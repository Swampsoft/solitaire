
use ggez::graphics::*;

use all::All;

use super::Component;
use super::GameState;
use super::types::*;

impl GameState {
    pub fn animation_update_system(&mut self, dt: f32) -> bool {
        let mut busy = false;

        let mut finished = Vec::new();

        for (p, a, e) in self.positions.iter_mut()
            .zip(self.animations.iter_mut())
            .zip(self.entities.iter())
            .filter_map(|x| x.all()) {
            if a.time_left > 0.0 {
                let time_step = dt.min(a.time_left);
                *p += (a.target_pos - *p) * time_step / a.time_left;
                a.time_left -= dt;
                busy = true;
            } else {
                finished.push(*e);
            }
        }

        for e in finished.into_iter() {
            let idx = self.ent_lookup[&e];
            if let Some(target) = self.animations[idx].take().unwrap().target_stack {
                let card = self.stacks[idx].take().unwrap().pop_card().unwrap();
                self.stacks[target].as_mut().unwrap().push_card(card);
            }
            self.remove_entity(e);
        }

        busy
    }
}