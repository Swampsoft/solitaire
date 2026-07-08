use ggez::event::*;
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::game::Game;
use crate::resources::Resources;

use super::main_state::MainState;
use ggez::graphics::{Canvas, DrawParam, Drawable};
use ggez::input::mouse::MouseButton;

pub struct VictoryState {
    pub resources: Resources,
    pub move_on: bool,
    pub game: Game,
}

impl EventHandler for VictoryState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = ctx.time.delta().as_secs_f32();
        self.game.state.run_update(dt, &mut self.resources);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, None);
        self.game
            .state
            .run_render(ctx, &mut self.resources, &mut canvas)?;

        let text = self.resources.get_text(ctx, "Congratulations.")?;
        let dim = text.dimensions(&ctx.gfx);
        let pos = Point2::from([640.0 - dim.w / 2.0, 403.0 - dim.h / 2.0]);
        canvas.draw(text, DrawParam::new().dest(pos));

        canvas.finish(&mut ctx.gfx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult<()> {
        ctx.request_quit();
        Ok(())
    }
}

impl From<MainState> for VictoryState {
    fn from(mut old: MainState) -> VictoryState {
        old.game.animate_victory();
        VictoryState {
            resources: old.resources,
            move_on: false,
            game: old.game,
        }
    }
}
