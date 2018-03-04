use ggez::{Context, GameResult};
use ggez::event::*;
use ggez::graphics;
use ggez::timer;

use game::Game;
use resources::Resources;
use table::Table;

use super::GameWrapper;
use super::main_state::MainState;

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
        let dt = timer::duration_to_f64(timer::get_delta(ctx)) as f32;
        self.game.state.run_update(dt, &mut self.resources);

        if !self.game.state.busy() {
            ctx.quit().unwrap();
        }

        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        self.game.state.run_render(ctx, &self.resources)?;

        graphics::present(ctx);
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