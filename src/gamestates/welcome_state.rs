
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::Point2;
use ggez::event::*;
use ggez::timer;

use super::GameWrapper;
use super::victory_state::VictoryState;
use super::giveup_state::GiveupState;

use cs::GameState;
use game::Game;
use resources::Resources;
use table::Table;

pub struct WelcomeState {
    pub resources: Resources,
    pub move_on: bool,
    pub game: Game,
    pub ready: bool,
}

impl WelcomeState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut table = Table::new();
        table.new_game();
        Ok(WelcomeState {
            resources: Resources::new(ctx)?,
            move_on: false,
            game: Game::new(),
            ready: false,
        })
    }

    pub fn next_state(self) -> GameWrapper{
        if self.move_on {
            GameWrapper::Game(self.into())
        } else {
            GameWrapper::Quit
        }
    }
}

impl EventHandler for WelcomeState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        //let t = timer::get_time_since_start(ctx);
        //self.table.update(t, &mut self.resources);

        if !self.ready {
            // skip first frame because it has a super high delta-time
            self.ready = true;
        } else {
            let dt = timer::duration_to_f64(timer::get_delta(ctx)) as f32;
            self.game.state.run_update(dt, &mut self.resources);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.state.run_render(ctx, &mut self.resources)?;

        //graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        //self.table.draw(ctx, &mut self.resources)?;

        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        let text = self.resources.get_text(ctx, "Click anywhere to start a new game.")?;
        let pos = Point2::new(640.0 - text.width() as f32 / 2.0, 403.0 - text.height() as f32 / 2.0);
        graphics::draw(ctx, text,pos, 0.0)?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        if !self.game.state.busy() {
            self.move_on = true;
            ctx.quit().unwrap();
        }
    }
}

impl From<VictoryState> for WelcomeState {
    fn from(mut old: VictoryState) -> WelcomeState {
        WelcomeState {
            resources: old.resources,
            move_on: false,
            game: Game::new(),
            ready: true,
        }
    }
}

impl From<GiveupState> for WelcomeState {
    fn from(mut old: GiveupState) -> WelcomeState {
        WelcomeState {
            resources: old.resources,
            move_on: false,
            game: Game::new(),
            ready: true,
        }
    }
}