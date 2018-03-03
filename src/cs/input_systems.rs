
use all::All;

use super::Component;
use super::GameState;
use super::types::*;

impl GameState {
    pub fn button_click_system(&mut self, click_pos: Point2) {
        let compound_iterator = self.positions.iter()
            .zip(self.buttons.iter_mut())
            .filter_map(|x| x.all());
        for (p, b) in compound_iterator {
            let dist = click_pos - p;
            if dist.norm_squared() <= BUTTON_RADIUS_SQUARED {
                b.state = ButtonState::Down;
            }
        }
    }

    pub fn begin_drag_system(&mut self, mouse_pos: Point2) {
        if self.drag_lock.is_some() {
            return
        }

        let mut hit = None;
        {
            let compound_iterator = self.positions.iter()
                .zip(self.stacks.iter_mut())
                .zip(self.entities.iter())
                .filter_map(|x| x.all());

            'outer:
                for (p, s, e) in compound_iterator {
                if mouse_pos.x < p.x || mouse_pos.y < p.y {
                    continue
                }

                for i in (0..s.len()).rev() {
                    let card_pos = p + s.get_stackshift() * i as f32;
                    if mouse_pos.x >= card_pos.x && mouse_pos.x <= card_pos.x + CARD_WIDTH &&
                        mouse_pos.y >= card_pos.y && mouse_pos.y <= card_pos.y + CARD_HEIGHT {

                        let substack = s.split(i);
                        hit = Some((card_pos, substack, *e));

                        break 'outer  // there can be only one
                    }
                }
            }
        }

        self.drag_lock = hit.map(|(p, s, e)| {
            (e, self.new_entity()
                .with_position(p)
                .with_stack(s)
                .with_zorder(200.0)
                .build())
        });
    }

    pub fn do_drag_system(&mut self, mouse_rel: Vector2) {
        if let Some((_, ent)) = self.drag_lock {
            *self.get_position_mut(ent).unwrap() += mouse_rel;
        }
    }

    pub fn done_drag_system(&mut self) {
        if let Some((src, drg)) = self.drag_lock.take() {
            let idx = self.ent_lookup[&drg];
            let d_stack = self.stacks[idx].take().unwrap();

            let stack = self.get_stack_mut(src).unwrap();
            stack.extend(d_stack);
        }
    }
}