
use std::iter::Chain;
use std::slice::{Iter, IterMut};
use std::ops::Range;

use rand::{thread_rng, Rng};

use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::Point2;

use button::{Button, ButtonState};
use cardstack::CardStack;
use cards::{Card, Color, Suite};
use resources::Resources;

pub struct Table {
    pub stacks: Vec<CardStack>,
    pub buttons: Vec<Button>,
}

impl Table {
    pub fn new() -> Table {
        let buttons = vec!{
            Button::new(Color::Red, Point2::new(533.0, 54.0)),
            Button::new(Color::Green, Point2::new(533.0, 137.0)),
            Button::new(Color::White, Point2::new(533.0, 220.0)),
        };

        let stacks = vec!{
            CardStack::new_dragon(45, 20),
            CardStack::new_dragon(197, 20),
            CardStack::new_dragon(349, 20),
            CardStack::new_target(805, 20),
            CardStack::new_target(957, 20),
            CardStack::new_target(1109, 20),
            CardStack::new_solitaire(45, 283),
            CardStack::new_solitaire(197, 283),
            CardStack::new_solitaire(349, 283),
            CardStack::new_solitaire(501, 283),
            CardStack::new_solitaire(653, 283),
            CardStack::new_solitaire(805, 283),
            CardStack::new_solitaire(957, 283),
            CardStack::new_solitaire(1109, 283),
            CardStack::new_rose(614, 20),
        };

        Table {
            buttons,
            stacks,
        }
    }

    pub fn dragon_stacks(&self) -> Range<usize> {
        0..3
    }

    pub fn target_stacks(&self) -> Range<usize> {
        3..6
    }

    pub fn solitaire_stacks(&self) -> Range<usize> {
        6..14
    }

    pub fn flower_stack(&self) -> usize {
        14
    }

    pub fn dragon_and_solitaire_stacks(&self) -> Chain<Range<usize>, Range<usize>> {
        self.dragon_stacks().chain(self.solitaire_stacks())
    }

    pub fn iter_solitaire_stacks(&self) -> Iter<CardStack> {
        self.stacks[self.solitaire_stacks()].iter()
    }

    pub fn iter_target_stacks(&self) -> Iter<CardStack> {
        let r = self.target_stacks();
        self.stacks[r].iter()
    }

    pub fn iter_mut_stacks(&mut self) -> IterMut<CardStack> {
        self.stacks.iter_mut()
    }

    pub fn get_stack(&self, i: usize) -> &CardStack {
        &self.stacks[i]
    }

    pub fn get_stack_mut(&mut self, i: usize) -> &mut CardStack {
        &mut self.stacks[i]
    }

    pub fn new_game(&mut self) {

        for button in self.buttons.iter_mut() {
            button.set_state(ButtonState::Up)
        }

        for stack in self.stacks.iter_mut() {
            stack.clear()
        }

        let mut cards = Vec::with_capacity(40);

        for i in 1..10 {
            cards.push(Card::new(Suite::Number(i, Color::Red)));
            cards.push(Card::new(Suite::Number(i, Color::Green)));
            cards.push(Card::new(Suite::Number(i, Color::White)));
        }

        for _ in 0..4 {
            cards.push(Card::new(Suite::Dragon(Color::Red)));
            cards.push(Card::new(Suite::Dragon(Color::Green)));
            cards.push(Card::new(Suite::Dragon(Color::White)));
        }

        cards.push(Card::new(Suite::Flower));

        thread_rng().shuffle(&mut cards);

        for (card, s) in cards.drain(..).zip(self.solitaire_stacks().cycle()) {
            self.stacks[s].push_card(card);
        }
    }

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult<()> {
        graphics::draw(ctx, &resources.table_image, Point2::new(0.0, 0.0), 0.0)?;

        for button in &self.buttons {
            button.draw(ctx, resources)?;
        }

        for stack in &self.stacks {
            stack.draw(ctx, resources)?;
        }

        Ok(())
    }

    pub fn find_dragon_target(&self, color: Color) -> Option<usize> {
        let mut target = None;
        for i in self.dragon_stacks() {
            match self.stacks[i].top_suite() {
                Some(&Suite::Dragon(c)) if c == color => return Some(i),
                None => target = Some(i),
                _ => {}
            }
        }
        target
    }
}
