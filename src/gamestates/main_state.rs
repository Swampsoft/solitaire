use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use game::Game;
use resources::Resources;

use ai::{AiResult, AiState};

use super::welcome_state::WelcomeState;
use super::GameWrapper;

pub struct MainState {
    pub resources: Resources,
    pub game: Game,
    win_counted: bool,
}

impl MainState {
    pub fn next_state(self) -> GameWrapper {
        if self.game.check_win_condition() {
            GameWrapper::Victory(self.into())
        } else {
            GameWrapper::GiveUp(self.into())
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.resources.music.playing() {
            self.resources.music.set_volume(0.5);
            self.resources.music.play()?;
        }

        let dt = timer::duration_to_f64(timer::delta(ctx)) as f32;
        self.game.state.run_update(dt, &mut self.resources);

        if self.game.check_win_condition() {
            if !self.win_counted {
                self.resources.add_win(ctx);
                self.win_counted = true;
            }
            ggez::event::quit(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.state.run_render(ctx, &mut self.resources)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.game
            .state
            .handle_mouse_button_down(x, y, &mut self.resources);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        self.game
            .state
            .handle_mouse_button_up(x, y, &mut self.resources);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, xrel: f32, yrel: f32) {
        self.game.state.handle_mouse_move(xrel, yrel);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => ggez::event::quit(ctx),
            KeyCode::Back => {
                let ai = AiState::new(self.game.export());
                match ai.astar(10000) {
                    AiResult::Unknown => println!("?"),
                    AiResult::Winable(n) => println!("{} :-)", n),
                    AiResult::Lost => println!(":-("),
                }
            }
            _ => {}
        }
    }
}

impl From<WelcomeState> for MainState {
    fn from(mut old: WelcomeState) -> MainState {
        old.game.animate_deal();
        MainState {
            resources: old.resources,
            game: old.game,
            win_counted: false,
        }
    }
}
