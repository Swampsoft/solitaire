
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::Point2;
use ggez::event::*;
use ggez::timer;

use super::GameWrapper;
use super::main_state::MainState;
use super::victory_state::VictoryState;

use resources::Resources;
use table::Table;

pub struct WelcomeState {
    pub resources: Resources,
    pub table: Table,
    pub move_on: bool,
}

impl WelcomeState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut table = Table::new();
        table.new_game();
        Ok(WelcomeState {
            resources: Resources::new(ctx)?,
            table: table,
            move_on: false,
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
        let t = timer::get_time_since_start(ctx);
        self.table.update(t, &mut self.resources);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        self.table.draw(ctx, &self.resources)?;

        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        let text = self.resources.get_text(ctx, "Click anywhere to start a new game.")?;
        let pos = Point2::new(640.0 - text.width() as f32 / 2.0, 403.0 - text.height() as f32 / 2.0);
        graphics::draw(ctx, text,pos, 0.0)?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        if self.table.game_enabled() {
            self.move_on = true;
            ctx.quit().unwrap();
        }
    }
}

impl From<VictoryState> for WelcomeState {
    fn from(old: VictoryState) -> WelcomeState {
        WelcomeState {
            resources: old.resources,
            table: old.table,
            move_on: false,
        }
    }
}

impl From<MainState> for WelcomeState {
    fn from(old: MainState) -> WelcomeState {
        WelcomeState {
            resources: old.resources,
            table: old.table,
            move_on: false,
        }
    }
}