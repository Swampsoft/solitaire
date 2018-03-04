
use all::All;

use super::Component;
use super::GameState;
use super::rules;
use super::types::*;

impl GameState {
    pub fn button_update_system(&mut self) {
        if !self.dirty || self.busy() {
            return
        }

        for b in self.buttons.iter_mut().filter_map(|x|x.all()) {
            if b.state == ButtonState::Down {
                continue
            }

            let target = self.stacks.iter().zip(self.entities.iter())   // iterate over stacks component of all entities
                .filter_map(|x| x.all())                    // filter those that have a stack
                .filter(|&(stack, _)| stack.role == StackRole::Dragon)
                .filter(|&(stack, _)| match stack.top() {
                    Some(Suite::Dragon(col)) => b.color == col,         // only dragons of right color
                    None => true,                                       // or empty stack
                    _ => false,
                })
                .map(|(stack, e)| *e)
                .next();

            b.target_stack = target;
            if target.is_none() {
                b.state = ButtonState::Up;
                b.source_stacks = Vec::new();
                continue
            }

            let sources = self.stacks.iter().zip(self.entities.iter())        // iterate over stacks component of all entities
                .filter_map(|x| x.all())                          // filter those that have a stack
                .filter_map(|(stack, e)| stack.top().map(|c| (c, e)))            // only top cards
                .filter(|&(card, _)| match card {                 // only dragons of right color
                    Suite::Dragon(col) => b.color == col,
                    _ => false,
                })
                .map(|(stack, e)| *e)
                .collect::<Vec<_>>();

            let n = sources.len();

            if n == 4 {
                b.state = ButtonState::Active;
                b.source_stacks = sources;
            } else {
                b.state = ButtonState::Up;
            }
        }
    }

    pub fn auto_move_system(&mut self) {
        if !self.dirty || self.busy() {
            return
        }
        self.dirty = false;

        let mut auto_move = None;

        {
            let flowers = self.stacks.iter().enumerate()
                .filter_map(|(i, s)| match *s {
                    Some(ref stack) => if stack.role == StackRole::Flower && stack.top().is_none() { Some(i) } else { None }
                    None => None
                })
                .collect::<Vec<_>>();

            let targets = self.stacks.iter().enumerate()
                .filter_map(|(i, s)| match *s {
                    Some(ref stack) => if stack.role == StackRole::Target { Some((i, stack.top())) } else { None }
                    None => None
                })
                .collect::<Vec<_>>();

            let top_cards = self.stacks.iter().zip(self.entities.iter())      // iterate over stacks component of all entities
                .filter_map(|x| x.all())                          // filter those that have a stack
                .filter_map(|(stack, e)| match stack.role {       // only top cards of Dragon and Generic stacks
                    StackRole::Dragon | StackRole::Sorting => stack.top().map(|c| (c, e)),
                    _ => None
                });

            'outer:
            for (c, s) in top_cards {
                match c {
                    Suite::Number(n, _) => {
                        let lowest = targets.iter().map(|&(_, l)| match l {
                            Some(Suite::Number(n, _)) => n,
                            None => 0,
                            _ => panic!("Invalid card on target stack")
                        }).min().unwrap();

                        if n > lowest + 1 {
                            continue 'outer
                        }

                        for &(t, _) in targets.iter() {
                            let d_stack = self.stacks[t].as_ref().unwrap();

                            if rules::is_valid_move(d_stack, c, 1) {
                                auto_move = Some((self.ent_lookup[s], t));
                                break 'outer
                            }
                        }
                    }
                    Suite::Flower => {
                        for f in flowers.iter() {
                            auto_move = Some((self.ent_lookup[s], *f));
                        }
                        break 'outer
                    }
                    _ => continue 'outer
                }
            }
        }

        if let Some((src, dst)) = auto_move {
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