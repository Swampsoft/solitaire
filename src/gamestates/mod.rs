mod main_state;
mod victory_state;
mod welcome_state;

use ggez::{Context, GameResult};
use ggez::event;
use sdl2::event::EventType;

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
        // make sure no unhandled events are left when entering a new state
        ctx.event_context.flush_events(EventType::First as u32, EventType::Last as u32);
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
