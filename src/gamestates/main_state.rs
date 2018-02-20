
use ggez::{Context, GameResult};
use ggez::event::*;
use ggez::graphics;
use ggez::timer;

use cardstack::CardStack;
use resources::Resources;
use rules;
use table::Table;

use super::GameWrapper;
use super::welcome_state::WelcomeState;

pub struct MainState {
    pub resources: Resources,
    pub table: Table,
    dragging: Option<CardStack>,
    dragsource: usize,
}

impl MainState {
    pub fn next_state(self) -> GameWrapper{
        if rules::check_wincondition(&self.table) {
            GameWrapper::Victory(self.into())
        } else {
            GameWrapper::GiveUp(self.into())
        }
    }
}

impl EventHandler for MainState  {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.resources.music.playing() {
            self.resources.music.set_volume(0.5);
            self.resources.music.play()?;
        }

        let t = timer::get_time_since_start(ctx);
        self.table.update(t, &mut self.resources);

        if !self.table.game_enabled() {
            return Ok(())
        }

        if rules::check_wincondition(&mut self.table) {
            ctx.quit()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        self.table.draw(ctx, &self.resources)?;

        if let Some(ref stack) = self.dragging {
            stack.draw(ctx, &self.resources)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: i32, y: i32) {
        if !self.table.game_enabled() {
            return
        }

        for (i, stack) in self.table.iter_mut_stacks().enumerate() {
            if let Some(s) = stack.start_drag(x as f32, y as f32) {
                self.dragsource = i;
                self.dragging = Some(s);
                self.resources.pickup_sound.play().unwrap();
                return
            }
        }

        self.table.handle_click(x as f32, y as f32);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        if !self.table.game_enabled() {
            return
        }

        if let Some(dstack) = self.dragging.take() {
            for s in 0..self.table.n_stacks() {
                if s == self.dragsource {
                    continue
                }
                if self.table.get_stack(s).accept_drop(&dstack) {
                    self.table.push_stack(s, dstack);
                    self.resources.place_sound.play().unwrap();
                    return
                }
            }
            self.resources.place_sound.play().unwrap();
            self.table.push_stack(self.dragsource, dstack);
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState,
                          _x: i32, _y: i32, xrel: i32, yrel: i32) {
        if let Some(ref mut stack) = self.dragging {
            stack.move_pos(xrel as f32, yrel as f32);
        }
    }
}

impl From<WelcomeState> for MainState {
    fn from(mut old: WelcomeState) -> MainState {
        old.table.animate_deal();
        MainState {
            resources: old.resources,
            table: old.table,
            dragsource: 0,
            dragging: None,
        }
    }
}
