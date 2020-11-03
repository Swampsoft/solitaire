use ggez::event::*;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use game::Game;
use resources::Resources;

use super::main_state::MainState;
use super::victory_state::VictoryState;
use super::GameWrapper;

pub struct GiveupState {
    pub resources: Resources,
    pub game: Game,
}

impl GiveupState {
    pub fn next_state(self) -> GameWrapper {
        GameWrapper::Welcome(self.into())
    }
}

impl EventHandler for GiveupState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::duration_to_f64(timer::delta(ctx)) as f32;
        self.game.state.run_update(dt, &mut self.resources);

        if !self.game.state.busy() {
            ggez::event::quit(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.state.run_render(ctx, &mut self.resources)?;

        graphics::present(ctx)?;
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
