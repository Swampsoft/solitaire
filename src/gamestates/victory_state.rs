use ggez::event::*;
use ggez::graphics;
use ggez::mint::Point2;
use ggez::timer;
use ggez::{Context, GameResult};

use game::Game;
use resources::Resources;

use super::main_state::MainState;
use super::GameWrapper;
use ggez::graphics::DrawParam;

pub struct VictoryState {
    pub resources: Resources,
    pub move_on: bool,
    pub game: Game,
}

impl VictoryState {
    pub fn next_state(self) -> GameWrapper {
        if self.move_on {
            GameWrapper::Welcome(self.into())
        } else {
            GameWrapper::GiveUp(self.into())
        }
    }
}

impl EventHandler for VictoryState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::duration_to_f64(timer::delta(ctx)) as f32;
        self.game.state.run_update(dt, &mut self.resources);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.state.run_render(ctx, &mut self.resources)?;

        let text = self.resources.get_text(ctx, "Congratulations.")?;
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
        ggez::event::quit(ctx);
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
