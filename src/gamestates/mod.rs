mod game;
mod main_state;
mod victory_state;
mod welcome_state;

use ggez::{Context, GameResult};
use ggez::event;

use self::welcome_state::WelcomeState;
use self::main_state::MainState;
use self::victory_state::VictoryState;

pub enum GameWrapper {
    Welcome(WelcomeState),
    Game(MainState),
    Victory(VictoryState),
    Quit,
}

impl GameWrapper {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameWrapper::Welcome(WelcomeState::new(ctx)?))
    }

    pub fn run(self, ctx: &mut Context) -> GameResult<Self> {
        use self::GameWrapper::*;
        match self {
            Welcome(mut state) => {
                event::run(ctx, &mut state)?;
                Ok(state.next_state())
            },
            Game(mut state) => {
                event::run(ctx, &mut state)?;
                Ok(state.next_state())
            },
            Victory(mut state) => {
                event::run(ctx, &mut state)?;
                Ok(state.next_state())
            },
            Quit => panic!("Invalid Game State")
        }
    }
}
