
use ggez::{Context, GameResult};
use ggez::event::*;
use ggez::graphics;

use cards::Suite;
use cardstack::CardStack;
use rules;

use super::GameWrapper;
use super::game::Game;
use super::welcome_state::WelcomeState;

pub struct MainState {
    dirty: bool,
    dragging: Option<CardStack>,
    dragsource: usize,
}

impl Game<MainState> {
    pub fn next_state(self) -> GameWrapper{
        GameWrapper::Quit
    }
}

impl EventHandler for Game<MainState>  {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while self.state.dirty {
            self.state.dirty = false;
            rules::global_rules(&mut self.table);
        }

        if rules::check_wincondition(&mut self.table) {
            ctx.quit()?;
        }

        if !self.resources.music.playing() {
            self.resources.music.set_volume(0.5);
            self.resources.music.play()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        self.table.draw(ctx, &self.resources)?;

        if let Some(ref stack) = self.state.dragging {
            stack.draw(ctx, &self.resources)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: i32, y: i32) {
        for (i, stack) in self.table.iter_mut_stacks().enumerate() {
            if let Some(s) = stack.start_drag(x as f32, y as f32) {
                self.state.dragsource = i;
                self.state.dragging = Some(s);
                self.resources.pickup_sound.play().unwrap();
                return
            }
        }
        let mut moves = Vec::new();
        for button in &self.table.buttons {
            if button.accept_click(x as f32, y as f32) {
                let t = self.table.find_dragon_target(button.color()).unwrap();
                for i in self.table.dragon_and_solitaire_stacks() {
                    if let Some(&Suite::Dragon(color)) = self.table.get_stack(i).top_suite() {
                        if color == button.color() {
                            moves.push((i, t));
                        }
                    }
                }
            }
        }
        for (s, t) in moves {
            let mut card = self.table.get_stack_mut(s).pop().unwrap();
            card.set_faceup(false);
            self.table.get_stack_mut(t).push_card(card);
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        if let Some(dstack) = self.state.dragging.take() {
            for (i, stack) in self.table.stacks.iter_mut().enumerate() {
                if i == self.state.dragsource {
                    continue
                }
                if stack.accept(&dstack) {
                    stack.push(dstack);
                    self.state.dirty = true;
                    self.resources.place_sound.play().unwrap();
                    return
                }
            }
            self.resources.place_sound.play().unwrap();
            self.table.stacks[self.state.dragsource].push(dstack);
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState,
                          _x: i32, _y: i32, xrel: i32, yrel: i32) {
        if let Some(ref mut stack) = self.state.dragging {
            stack.move_pos(xrel as f32, yrel as f32);
        }
    }
}

impl From<Game<WelcomeState>> for Game<MainState> {
    fn from(mut old: Game<WelcomeState>) -> Game<MainState> {
        old.table.new_game();
        Game {
            resources: old.resources,
            table: old.table,
            state: MainState {
                dirty: true,
                dragsource: 0,
                dragging: None,
            },
        }
    }
}
