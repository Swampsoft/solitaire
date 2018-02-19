
use ggez::{Context, GameResult};
use ggez::event::*;

use super::GameWrapper;

pub struct VictoryState {
}

impl VictoryState {
    pub fn next_state(self) -> GameWrapper{
        GameWrapper::Quit
    }
}

impl EventHandler for VictoryState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        unimplemented!()
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        unimplemented!()
    }
}
