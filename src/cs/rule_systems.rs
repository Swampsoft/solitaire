
use all::All;

use super::Component;
use super::GameState;
use super::types::*;

impl GameState {
    pub fn button_update_system(&mut self) {
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
}