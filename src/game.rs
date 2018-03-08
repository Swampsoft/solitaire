use std::iter;
use std::slice;

use rand::{thread_rng, Rng};

use cs::GameState;
use types::*;

pub struct Game {
    pub state: GameState,

    flower_stack: Entity,
    all_stacks: Vec<Entity>,
    game_stacks: Vec<Entity>,
    target_stacks: [Entity; 3],
    //dragon_stacks: [Entity; 3],
}

impl Game {
    pub fn new() -> Game {
        let mut state = GameState::default();

        state.new_entity().with_position(Point2::new(533.0, 54.0)).with_button(Button::new(Color::Red)).build();
        state.new_entity().with_position(Point2::new(533.0, 137.0)).with_button(Button::new(Color::Green)).build();
        state.new_entity().with_position(Point2::new(533.0, 220.0)).with_button(Button::new(Color::White)).build();

        let r = state.new_entity().with_position(Point2::new(45.0, 20.0)).with_stack(Stack::new(StackRole::Dragon)).build();
        let s = state.new_entity().with_position(Point2::new(197.0, 20.0)).with_stack(Stack::new(StackRole::Dragon)).build();
        let t = state.new_entity().with_position(Point2::new(349.0, 20.0)).with_stack(Stack::new(StackRole::Dragon)).build();
        let flower_stack = state.new_entity().with_position(Point2::new(614.0, 20.0)).with_stack(Stack::new(StackRole::Flower)).build();
        let x = state.new_entity().with_position(Point2::new(805.0, 20.0)).with_stack(Stack::new(StackRole::Target)).build();
        let y = state.new_entity().with_position(Point2::new(957.0, 20.0)).with_stack(Stack::new(StackRole::Target)).build();
        let z = state.new_entity().with_position(Point2::new(1109.0, 20.0)).with_stack(Stack::new(StackRole::Target)).build();
        let a = state.new_entity().with_position(Point2::new(45.0, 283.0)).with_stack(Stack::new(StackRole::Sorting)).build();
        let b = state.new_entity().with_position(Point2::new(197.0, 283.0)).with_stack(Stack::new(StackRole::Sorting)).build();
        let c = state.new_entity().with_position(Point2::new(349.0, 283.0)).with_stack(Stack::new(StackRole::Sorting)).build();
        let d = state.new_entity().with_position(Point2::new(501.0, 283.0)).with_stack(Stack::new(StackRole::Sorting)).build();
        let e = state.new_entity().with_position(Point2::new(653.0, 283.0)).with_stack(Stack::new(StackRole::Sorting)).build();
        let f = state.new_entity().with_position(Point2::new(805.0, 283.0)).with_stack(Stack::new(StackRole::Sorting)).build();
        let g = state.new_entity().with_position(Point2::new(957.0, 283.0)).with_stack(Stack::new(StackRole::Sorting)).build();
        let h = state.new_entity().with_position(Point2::new(1109.0, 283.0)).with_stack(Stack::new(StackRole::Sorting)).build();

        let mut game = Game {
            state,
            flower_stack,
            all_stacks: vec!(a, b, c, d, e, f, g, h, r, s, t, flower_stack, x, y, z),
            game_stacks: vec!(a, b, c, d, e, f, g, h),
            target_stacks: [x, y, z],
            //dragon_stacks: [r, s, t],
        };

        game.animate_shuffle();

        game
    }

    pub fn export<'a>(&'a self) -> Vec<Stack> {
        self.all_stacks.iter()
            .map(|e| (*self.state.get_stack(*e).unwrap()).clone())
            .collect()
    }

    pub fn check_win_condition(&self) -> bool {
        self.game_stacks.iter().all(|&s| self.state.get_stack(s).unwrap().len() == 0)
        && self.target_stacks.iter().all(|&s| self.state.get_stack(s).unwrap().len() == 9)
    }

    pub fn shuffled_deck() -> Stack {
        let mut cards = Vec::with_capacity(40);

        for _ in 0..4 {
            cards.push(Suite::Dragon(Color::Red));
            cards.push(Suite::Dragon(Color::Green));
            cards.push(Suite::Dragon(Color::White));
        }

        for i in 1..10 {
            cards.push(Suite::Number(i, Color::Red));
            cards.push(Suite::Number(i, Color::Green));
            cards.push(Suite::Number(i, Color::White));
        }

        cards.push(Suite::Flower);

        thread_rng().shuffle(&mut cards);

        Stack {
            cards,
            role: StackRole::Generic,
        }
    }

    pub fn animate_shuffle(&mut self) {
        let f = self.flower_stack;
        let target_stack = Some(f);
        let stack_pos = *self.state.get_position(f).unwrap();
        let shift = self.state.get_stack(f).unwrap().get_stackshift();
        for n in 0..40 {
            let i = 1.0 + 0.1 * (n as f32);
            let start_pos = stack_pos - shift * i * (stack_pos.y + CARD_HEIGHT) / shift.y;
            let target_pos = stack_pos + shift * n as f32;
            let sound_stop = if n % 10 == 0 {
                Sounds::Place
            } else {
                Sounds::None
            };
            let ani = Animation { target_pos, target_stack, start_delay: 0.0, time_left: 0.1 * i, sound_start: Sounds::None, sound_stop };
            self.state.animate(Suite::FaceDown, start_pos, 100.0 + n as f32, ani);
        }
    }

    pub fn animate_deal(&mut self) {
        self.state.get_stack_mut(self.flower_stack).unwrap().cards.clear();

        let mut new_deck = Game::shuffled_deck();  // TODO: should this be a parameter to the function?

        let fpos = *self.state.get_position(self.flower_stack).unwrap();
        let fshift = self.state.get_stack(self.flower_stack).unwrap().get_stackshift();

        let mut height = 0.0;
        let mut s = 0;
        let mut z= new_deck.len() as f32;
        let mut start_delay = 0.0;
        while let Some(card) = new_deck.pop_card() {
            let target_stack = self.game_stacks[s];
            let shift = self.state.get_stack(target_stack).unwrap().get_stackshift();
            let target_pos = *self.state.get_position(target_stack).unwrap() + shift * height;

            let start_pos = fpos + fshift * height;

            let ani = Animation {target_pos, target_stack: Some(target_stack), start_delay, time_left: 0.2, sound_start: Sounds::Place, sound_stop: Sounds::None};
            self.state.animate(card, start_pos, 100.0 + z, ani);

            s += 1;
            if s >= self.game_stacks.len() {
                s = 0;
                height += 1.0;
            }

            z -= 1.0;
            start_delay += 0.1;
        }
    }

    pub fn animate_giveup(&mut self) {
        let mut cards = Vec::with_capacity(40);

        for &e in self.state.iter() {
            let pos = match self.state.get_position(e) {
                Some(p) => p,
                None => continue,
            };

            let stack = match self.state.get_stack(e) {
                Some(s) => s,
                None => continue,
            };

            for (i, &card) in stack.iter().enumerate() {
                let cardpos = pos + stack.get_stackshift() * i as f32;
                cards.push((card, cardpos));
            }
        }

        self.state.clear();

        for (z, (card, start_pos)) in cards.into_iter().enumerate() {

            let mut direction = start_pos - Point2::new(640.0, 400.0);
            let dist = direction.norm();
            direction = direction / dist;

            let target_pos = start_pos + direction * 800.0;

            let ani = Animation {target_pos, target_stack: None, start_delay: 0.0, time_left: 0.2, sound_start: Sounds::None, sound_stop: Sounds::None};
            self.state.animate(card, start_pos, 100.0 + z as f32, ani);
        }
    }

    pub fn animate_victory(&mut self) {
        let mut cards = Vec::with_capacity(40);

        loop {
            let mut empty = true;
            for &e in &self.all_stacks {
                let pos = *self.state.get_position(e).unwrap();
                let stack = self.state.get_stack_mut(e).unwrap();
                if let Some(card) = stack.pop_card() {
                    empty = false;
                    let cardpos = pos + stack.get_stackshift() * stack.len() as f32;
                    cards.push((card, cardpos));
                    continue
                }
            }
            if empty { break }
        }

        for (z, (card, start_pos)) in cards.into_iter().enumerate() {
            let target_pos = start_pos + Vector2::new(0.0, 800.0);

            let ani = Animation {target_pos, target_stack: None, start_delay: 0.3 * z as f32, time_left: 3.0, sound_start: Sounds::None, sound_stop: Sounds::None};
            self.state.animate(card, start_pos, 500.0 - z as f32, ani);
        }
    }
}