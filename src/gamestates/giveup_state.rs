use ggez::{Context, GameResult};
use ggez::event::*;
use ggez::graphics;
use ggez::timer;

use resources::Resources;
use table::Table;

use super::GameWrapper;
use super::main_state::MainState;

pub struct GiveupState {
    pub resources: Resources,
    pub table: Table,
}

impl GiveupState {
    pub fn next_state(self) -> GameWrapper {
        GameWrapper::Welcome(self.into())
    }
}

impl EventHandler for GiveupState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let t = timer::get_time_since_start(ctx);
        self.table.update(t, &mut self.resources);

        if self.table.game_enabled() {
            ctx.quit().unwrap();
        }

        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        self.table.draw(ctx, &self.resources)?;

        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;

        graphics::present(ctx);
        Ok(())
    }
}

impl From<MainState> for GiveupState {
    fn from(mut old: MainState) -> GiveupState {
        old.table.animate_giveup();
        GiveupState {
            resources: old.resources,
            table: old.table,
        }
    }
}