
use utils::all::All;
use rules;
use types::*;

use super::GameState;

impl GameState {
    pub fn button_update_system(&mut self) {
        if !self.dirty || self.busy() {
            return
        }

        for b in self.buttons.iter_mut().filter_map(|x|x.all()) {
            if b.state == ButtonState::Down {
                continue
            }

            let (stacks, ents): (Vec<_>, Vec<Entity>) = self.stacks.iter().zip(self.entities.iter())
                .filter_map(|(stack, e)| stack.as_ref().map(|s| (s, e)))
                .unzip();

            let r = rules::check_button(b.color, stacks.into_iter());

            match r {
                None => {
                    b.state = ButtonState::Up;
                    b.stacks = None;
                    continue
                }
                Some((t, s)) => {
                    b.state = ButtonState::Active;
                    b.stacks = Some((ents[t],[ents[s[0]], ents[s[1]], ents[s[2]], ents[s[3]]]))
                }
            }
        }
    }

    pub fn auto_move_system(&mut self) {
        if !self.dirty || self.busy() {
            return
        }
        self.dirty = false;

        let auto_move;
        {
            let stacks: Vec<_> = self.stacks.iter()
                .filter_map(|s| s.as_ref())
                .collect();

            let (stacks, idx): (Vec<_>, Vec<_>) = self.stacks.iter().enumerate()
                .filter_map(|(i, stack)| stack.as_ref().map(|s| (s, i)))
                .unzip();

            auto_move = rules::get_automove(stacks.into_iter())
                .map(|(t, s)| (idx[t], idx[s]));
        }

        if let Some((dst, src)) = auto_move {
            let card;
            let start_pos;
            let target_pos;
            let target_stack;
            {
                let s_stack = self.stacks[src].as_mut().unwrap();
                card = s_stack.pop_card().unwrap();
            }
            {
                let s_stack = self.stacks[src].as_ref().unwrap();
                let s_pos = self.positions[src].as_ref().unwrap();
                start_pos = s_pos + s_stack.get_stackshift() * s_stack.len() as f32;

                target_stack = Some(self.entities[dst]);
                let t_stack = self.stacks[dst].as_ref().unwrap();
                let t_pos = self.positions[dst].as_ref().unwrap();
                target_pos = t_pos + t_stack.get_stackshift() * t_stack.len() as f32;
            }

            let ani = Animation {target_pos, target_stack, start_delay: 0.0, time_left: 0.3, sound_start: Sounds::Sweep, sound_stop: Sounds::None};
            self.animate(card, start_pos, 100.0, ani);
        }
    }
}