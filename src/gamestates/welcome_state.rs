use ggez::event::*;
use ggez::graphics;
use ggez::mint::Point2;
use ggez::timer;
use ggez::{Context, GameResult};

use super::giveup_state::GiveupState;
use super::victory_state::VictoryState;
use super::GameWrapper;

use game::Game;
use ggez::graphics::DrawParam;
use resources::Resources;

pub struct WelcomeState {
    pub resources: Resources,
    pub move_on: bool,
    pub game: Game,
    pub ready: bool,
}

impl WelcomeState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(WelcomeState {
            resources: Resources::new(ctx)?,
            move_on: false,
            game: Game::new(),
            ready: false,
        })
    }

    pub fn next_state(self) -> GameWrapper {
        if self.move_on {
            GameWrapper::Game(self.into())
        } else {
            GameWrapper::Quit
        }
    }
}

impl EventHandler for WelcomeState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.ready {
            // skip first frame because it has a super high delta-time
            self.ready = true;
        } else {
            let dt = timer::duration_to_f64(timer::delta(ctx)) as f32;
            self.game.state.run_update(dt, &mut self.resources);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.state.run_render(ctx, &mut self.resources)?;

        let text = self
            .resources
            .get_text(ctx, "Click anywhere to start a new game.")?;
        let pos = Point2::from([
            640.0 - text.width(ctx) as f32 / 2.0,
            403.0 - text.height(ctx) as f32 / 2.0,
        ]);
        graphics::draw(ctx, text, DrawParam::new().dest(pos))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if !self.game.state.busy() {
            self.move_on = true;
            ggez::event::quit(ctx);
        }
    }
}

impl From<VictoryState> for WelcomeState {
    fn from(old: VictoryState) -> WelcomeState {
        WelcomeState {
            resources: old.resources,
            move_on: false,
            game: Game::new(),
            ready: true,
        }
    }
}

impl From<GiveupState> for WelcomeState {
    fn from(old: GiveupState) -> WelcomeState {
        WelcomeState {
            resources: old.resources,
            move_on: false,
            game: Game::new(),
            ready: true,
        }
    }
}
