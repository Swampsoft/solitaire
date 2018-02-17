
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::Point2;
use ggez::event::*;

use super::GameWrapper;
use super::game::Game;

use resources::Resources;
use table::Table;

pub struct WelcomeState {
    move_on: bool,
}

impl Game<WelcomeState> {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Game {
            resources: Resources::new(ctx)?,
            table: Table::new(),
            state: WelcomeState {
                move_on: false,
            }
        })
    }

    pub fn next_state(self) -> GameWrapper{
        if self.state.move_on {
            GameWrapper::Game(self.into())
        } else {
            GameWrapper::Quit
        }
    }
}

impl EventHandler for Game<WelcomeState> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.table.draw(ctx, &self.resources)?;

        let text = self.resources.get_text(ctx, "Click anywhere to start a new game.")?;
        let pos = Point2::new(640.0 - text.width() as f32 / 2.0, 403.0 - text.height() as f32 / 2.0);
        graphics::draw(ctx, text,pos, 0.0)?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        self.state.move_on = true;
        ctx.quit().unwrap();
    }
}
