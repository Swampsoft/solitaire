
use ggez::{Context, GameResult};
use ggez::event::*;

use super::GameWrapper;
use super::game::Game;

pub struct VictoryState {
}

impl Game<VictoryState> {
    pub fn next_state(self) -> GameWrapper{
        GameWrapper::Quit
    }
}

impl EventHandler for Game<VictoryState> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        unimplemented!()
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        unimplemented!()
    }
}
