
use all::All;
use bbox::BoundingBox;

use super::Component;
use super::GameState;
use super::rules;
use super::types::*;

impl GameState {
    pub fn button_click_system(&mut self, click_pos: Point2) {
        let mut animation = Vec::new();
        {
            let compound_iterator = self.positions.iter()
                .zip(self.buttons.iter_mut())
                .filter_map(|x| x.all())
                .filter(|&(_, ref b)| b.state == ButtonState::Active);
            'outer:
                for (p, b) in compound_iterator {
                let dist = click_pos - p;
                if dist.norm_squared() <= BUTTON_RADIUS_SQUARED {
                    b.state = ButtonState::Down;
                    let t = self.ent_lookup[&b.target_stack.unwrap()];
                    let target_pos = self.positions[t].unwrap();
                    for e in b.source_stacks.drain(..) {
                        let s = self.ent_lookup[&e];
                        let stack = self.stacks[s].as_mut().unwrap();
                        let pos = &self.positions[s].unwrap();

                        //self.stacks[t].as_mut().unwrap().push_card(c);

                        let start_pos = pos + stack.get_stackshift() * (stack.len() - 1) as f32;
                        stack.pop_card();

                        let ani = Animation { target_pos, target_stack: b.target_stack, start_delay: 0.0, time_left: 0.1 };
                        animation.push((start_pos, ani));
                    }
                    break 'outer
                }
            }
        }
        for (start_pos, ani) in animation {
            self.animate(Suite::FaceDown, start_pos, 100.0, ani);
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

                        if rules::is_valid_drag(s, i) {
                            let substack = s.split(i);
                            hit = Some((card_pos, substack, *e));
                        }
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
            let mut d_stack = self.stacks[idx].take();
            let pos = self.positions[idx].take().unwrap();

            let bb_drag = BoundingBox::new(pos.x, pos.x + CARD_WIDTH, pos.y, pos.y + CARD_HEIGHT);

            {
                let compound_iterator = self.positions.iter()
                    .zip(self.stacks.iter_mut())
                    .filter_map(|x| x.all());
                for (p, s) in compound_iterator {
                    let q = p + s.get_stackshift() * (s.len() as f32 - 1.0).max(0.0) + Vector2::new(CARD_WIDTH, CARD_HEIGHT);
                    let bb_target = BoundingBox::new(p.x, q.x, p.y, q.y);

                    if bb_target.intersects(&bb_drag) {
                        if rules::is_valid_drop(s, d_stack.as_ref().unwrap()) {
                            s.extend(d_stack.take().unwrap());
                            break
                        }
                    }
                }
            }

            if let Some(ds) = d_stack {
                let stack = self.get_stack_mut(src).unwrap();
                stack.extend(ds);
            }

            self.remove_entity(drg);
        }
    }
}