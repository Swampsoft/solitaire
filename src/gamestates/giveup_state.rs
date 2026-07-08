use crate::game::Game;
use crate::resources::Resources;
use ggez::event::*;
use ggez::graphics::Canvas;
use ggez::{Context, GameResult};

use super::main_state::MainState;
use super::victory_state::VictoryState;

pub struct GiveupState {
    pub resources: Resources,
    pub game: Game,
}

impl EventHandler for GiveupState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        self.game.state.run_update(dt, &mut self.resources);

        if !self.game.state.busy() {
            ctx.request_quit();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(&ctx.gfx, None);
        self.game
            .state
            .run_render(ctx, &mut self.resources, &mut canvas)?;
        canvas.finish(&mut ctx.gfx)?;
        Ok(())
    }
}

impl From<MainState> for GiveupState {
    fn from(mut old: MainState) -> GiveupState {
        old.game.animate_giveup();
        GiveupState {
            resources: old.resources,
            game: old.game,
        }
    }
}

impl From<VictoryState> for GiveupState {
    fn from(mut old: VictoryState) -> GiveupState {
        old.game.animate_giveup();
        GiveupState {
            resources: old.resources,
            game: old.game,
        }
    }
}
