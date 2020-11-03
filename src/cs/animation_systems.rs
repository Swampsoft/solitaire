use types::*;
use utils::all::All;

use super::GameState;

use resources::Resources;

impl GameState {
    pub fn animation_update_system(&mut self, dt: f32, res: &mut Resources) -> bool {
        let mut busy = false;

        let mut finished = Vec::new();

        for (p, a, e) in self
            .positions
            .iter_mut()
            .zip(self.animations.iter_mut())
            .zip(self.entities.iter())
            .filter_map(|x| x.all())
        {
            if a.time_left > 0.0 {
                busy = true;
                let dt = if a.start_delay > 0.0 {
                    let step = dt.min(a.start_delay);
                    a.start_delay -= step;
                    dt - step
                } else {
                    dt
                };
                if a.sound_start != Sounds::None && a.start_delay <= 0.0 {
                    res.play_sound(a.sound_start);
                    a.sound_start = Sounds::None;
                }
                let time_step = dt.min(a.time_left);
                *p += (a.target_pos - *p) * time_step / a.time_left;
                a.time_left -= dt;
            } else {
                finished.push(*e);
                res.play_sound(a.sound_stop);
            }
        }

        for e in finished.into_iter() {
            let idx = self.ent_lookup[&e];
            if let Some(target) = self.animations[idx].take().unwrap().target_stack {
                let tidx = self.ent_lookup[&target];
                let card = self.stacks[idx].take().unwrap().pop_card().unwrap();
                self.stacks[tidx].as_mut().unwrap().push_card(card);
                self.dirty = true;
            }
            self.remove_entity(e);
        }

        busy
    }
}
