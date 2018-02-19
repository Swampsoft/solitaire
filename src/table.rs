
use std::iter::Chain;
use std::ops::Range;
use std::slice::{Iter, IterMut};
use std::time;

use rand::{thread_rng, Rng};

use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::Point2;

use animation::{Animation, AnimationHandler};
use button::{Button, ButtonState};
use cardstack::CardStack;
use cards::{Card, Color, Suite};
use resources::{Resources, Sounds};
use rules;


const ANIMATION_DURATION: u32 =  200_000_000;  // nano seconds
const DEAL_INTERVAL: u32 = 100_000_000;


pub struct Table {
    dirty: bool,
    stacks: Vec<CardStack>,
    buttons: Vec<Button>,
    animations: AnimationHandler,
    deal_pending: bool,
    animove: Option<(usize, usize)>,
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
            dirty: true,
            buttons,
            stacks,
            animations: AnimationHandler::new(),
            deal_pending: false,
            animove: None,
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

    /*pub fn get_stack_mut(&mut self, i: usize) -> &mut CardStack {
        &mut self.stacks[i]
    }*/

    pub fn push_stack(&mut self, i: usize, substack: CardStack) {
        self.stacks[i].push(substack);
        self.dirty = true;
    }

    pub fn n_stacks(&self)-> usize {
        self.stacks.len()
    }

    pub fn get_button(&self, i: usize) -> &Button {
        &self.buttons[i]
    }

    pub fn set_button(&mut self, i: usize, state: ButtonState) {
        let button = &mut self.buttons[i];
        if button.state() != state {
            self.dirty = true;
        }
        button.set_state(state);
    }

    pub fn n_buttons(&self) -> usize {
        self.buttons.len()
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

        /*for (card, s) in cards.drain(..).zip(self.solitaire_stacks().cycle()) {
            self.stacks[s].push_card(card);
        }*/

        let s = self.flower_stack();
        for card in cards.drain(..) {
            self.stacks[s].push_card(card);
        }
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn game_enabled(&self) -> bool {
        !self.animations.busy()
    }

    pub fn update(&mut self, t_now: time::Duration, res: &mut Resources) {
        if self.deal_pending {
            self.deal_pending = false;
            self.schedule_deal(t_now);
        }
        if self.animove.is_some() {
            self.schedule_move(t_now);
        }
        for (card, t) in self.animations.update(t_now, res) {
            // see if the animation engine returned any cards to the table...
            self.stacks[t].push_card(card);
            self.set_dirty();
        }

        while self.dirty && self.game_enabled() {
            self.dirty = rules::global_rules(self);
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

        self.animations.draw(ctx, resources)?;

        Ok(())
    }

    pub fn handle_click(&mut self, x: f32, y:f32) {
        let mut moves = Vec::new();
        for b in 0..self.n_buttons() {
            if self.buttons[b].accept_click(x, y) {
                let t = self.find_dragon_target(self.buttons[b].color()).unwrap();
                for i in self.dragon_and_solitaire_stacks() {
                    if let Some(&Suite::Dragon(color)) = self.get_stack(i).top_suite() {
                        if color == self.buttons[b].color() {
                            moves.push((i, t));
                        }
                    }
                }
                self.buttons[b].set_state(ButtonState::Down);
                self.set_dirty();
            }
        }
        for (s, t) in moves {
            let mut card = self.stacks[s].pop().unwrap();
            card.set_faceup(false);
            self.stacks[t].push_card(card);
        }
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

    pub fn deal(&mut self) {
        self.deal_pending = true
    }

    pub fn schedule_deal(&mut self, mut t_start: time::Duration) {
        let s = self.flower_stack();
        assert_eq!(self.stacks[s].len(), 40);

        let mut virtual_stacks = vec![0; self.stacks.len()];

        let mut t_stop;

        for t in self.solitaire_stacks().cycle() {
            let card = match self.stacks[s].pop() {
                Some(card) => card,
                None => break,
            };

            let n = virtual_stacks[t];
            let dest = self.stacks[t].calc_card_pos(n);
            virtual_stacks[t] += 1;  // push virtual cards on virtual stack

            t_stop = t_start + time::Duration::new(0, ANIMATION_DURATION);

            let anim = Animation::new(card, dest, t_start, t_stop, t, Sounds::Place, Sounds::None);
            self.animations.add(anim);

            t_start = t_start + time::Duration::new(0, DEAL_INTERVAL);
        }
    }

    pub fn move_card(&mut self, src: usize, dst: usize) {
        self.animove = Some((src, dst));
    }

    pub fn schedule_move(&mut self, t_start: time::Duration) {
        if let Some((src, dst)) = self.animove.take() {
            let card = self.stacks[src].pop().unwrap();
            let dest = self.stacks[dst].calc_new_pos();

            let t_stop = t_start + time::Duration::new(0, ANIMATION_DURATION);

            let anim = Animation::new(card, dest, t_start, t_stop, dst, Sounds::Sweep, Sounds::None);
            self.animations.add(anim);
        }
    }
}
