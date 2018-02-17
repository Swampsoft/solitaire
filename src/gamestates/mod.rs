mod game;
mod main_state;
mod victory_state;
mod welcome_state;

use ggez::{Context, GameResult};
use ggez::event;

use self::game::Game;
use self::welcome_state::WelcomeState;
use self::main_state::MainState;
use self::victory_state::VictoryState;

/*pub struct GameRunner {
    state: GameWrapper,
}

impl GameRunner {
    pub fn new(ctx: &mut Context) -> GameResult<GameRunner> {
        let runner = GameRunner {
            state: Game::new(ctx)?
        };
        Ok(runner)
    }*
}*/

/*impl EventHandler for GameRunner {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let state = unsafe {
            // We leave `self.state` in an uninitialized memory state.
            // This memory must be replaced with something valid before leaving the function, without calling `drop` on it.
            ::std::mem::replace(&mut self.state, ::std::mem::uninitialized())
        };

        // Don't use `?` here, because in case of an error this would return immediately,
        // dropping the uninitialized memory we left in `self.state`.
        let result = state.update(ctx);

        match result {
            Ok(newstate) => {
                let uninitialized_memory = ::std::mem::replace(&mut self.state, newstate);
                ::std::mem::forget(uninitialized_memory);
                Ok(())
            },
            Err(e) => {
                // In case of error the old state was lost, so we must make something up.
                let uninitialized_memory = ::std::mem::replace(&mut self.state, GameWrapper::DummyState);
                ::std::mem::forget(uninitialized_memory);
                Err(e)
            },
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.state.draw(ctx)
    }
}*/

pub enum GameWrapper {
    Welcome(Game<WelcomeState>),
    Game(Game<MainState>),
    Victory(Game<VictoryState>),
    Quit,
}

impl GameWrapper {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameWrapper::Welcome(Game::new(ctx)?))
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
