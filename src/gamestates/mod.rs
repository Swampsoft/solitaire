mod giveup_state;
mod main_state;
mod victory_state;
mod welcome_state;

use ggez::{Context, GameError, GameResult};
use std::fmt;
use std::fmt::Formatter;

use self::giveup_state::GiveupState;
use self::main_state::MainState;
use self::victory_state::VictoryState;
use self::welcome_state::WelcomeState;

pub struct GameWrapper {
    state: GameState,
}
#[derive(Default)]
enum GameState {
    Welcome(WelcomeState),
    Game(MainState),
    Victory(VictoryState),
    GiveUp(GiveupState),
    #[default]
    Quit,
}

use self::GameState::*;
use ggez::event::EventHandler;
use ggez::input::keyboard::KeyInput;
use ggez::input::mouse::MouseButton;
use log::info;

impl GameWrapper {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let state = Welcome(WelcomeState::new(ctx)?);
        info!("Entering game state {}", state);
        Ok(GameWrapper { state })
    }

    fn handler(&mut self) -> &mut dyn EventHandler {
        match &mut self.state {
            Welcome(s) => s,
            Game(s) => s,
            Victory(s) => s,
            GiveUp(s) => s,
            Quit => panic!("Invalid Game State"),
        }
    }

    fn advance(&mut self) {
        info!("Leaving game state {}", self.state);
        match std::mem::take(&mut self.state) {
            Welcome(state) => {
                if state.move_on {
                    self.state = Game(state.into());
                }
            }
            Game(state) => {
                if state.game.check_win_condition() {
                    self.state = Victory(state.into());
                } else {
                    self.state = GiveUp(state.into());
                }
            }
            Victory(state) => {
                if state.move_on {
                    self.state = Welcome(state.into());
                } else {
                    self.state = GiveUp(state.into());
                }
            }
            GiveUp(state) => {
                self.state = Welcome(state.into());
            }
            Quit => {}
        }
        info!("Entering game state {}", self.state);
    }
}

impl EventHandler for GameWrapper {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if matches!(self.state, Quit) {
            return Ok(());
        }
        self.handler().update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if matches!(self.state, Quit) {
            return Ok(());
        }
        self.handler().draw(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.handler().mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.handler().mouse_button_up_event(ctx, button, x, y)
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) -> GameResult {
        self.handler().mouse_motion_event(ctx, x, y, dx, dy)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
        self.handler().key_down_event(ctx, input, repeated)
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        self.advance();
        Ok(!matches!(self.state, Quit))
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Welcome(_) => "Welcome",
            Game(_) => "Game",
            Victory(_) => "Victory",
            GiveUp(_) => "GiveUp",
            Quit => "Quit",
        };
        write!(f, "{}", name)
    }
}
