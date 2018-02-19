
use ggez::{Context, GameResult};
use ggez::event::*;
use ggez::graphics;
use ggez::graphics::Point2;

use resources::Resources;
use table::Table;

use super::GameWrapper;
use super::main_state::MainState;

pub struct VictoryState {
    pub resources: Resources,
    pub table: Table,
    pub move_on: bool,
}

impl VictoryState {
    pub fn next_state(self) -> GameWrapper {
        if self.move_on {
            GameWrapper::Welcome(self.into())
        } else {
            GameWrapper::Quit
        }
    }
}

impl EventHandler for VictoryState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        self.table.draw(ctx, &self.resources)?;

        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        let text = self.resources.get_text(ctx, "Congratulations.")?;
        let pos = Point2::new(640.0 - text.width() as f32 / 2.0, 403.0 - text.height() as f32 / 2.0);
        graphics::draw(ctx, text,pos, 0.0)?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        self.move_on = true;
        ctx.quit().unwrap();
    }
}

impl From<MainState> for VictoryState {
    fn from(old: MainState) -> VictoryState {
        VictoryState {
            resources: old.resources,
            table: old.table,
            move_on: false,
        }
    }
}