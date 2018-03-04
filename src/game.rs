
use rand::{thread_rng, Rng};

use cs::GameState;
use cs::types::*;

pub struct Game {
    pub state: GameState,

    flower_stack: Entity,
    game_stacks: Vec<Entity>,
}

impl Game {
    pub fn new() -> Game {
        let mut state = GameState::default();

        state.new_entity().with_position(Point2::new(533.0, 54.0)).with_button(Button::new(Color::Red)).build();
        state.new_entity().with_position(Point2::new(533.0, 137.0)).with_button(Button::new(Color::Green)).build();
        state.new_entity().with_position(Point2::new(533.0, 220.0)).with_button(Button::new(Color::White)).build();

        state.new_entity().with_position(Point2::new(45.0, 20.0)).with_stack(Stack::new(StackRole::Dragon)).build();
        state.new_entity().with_position(Point2::new(197.0, 20.0)).with_stack(Stack::new(StackRole::Dragon)).build();
        state.new_entity().with_position(Point2::new(349.0, 20.0)).with_stack(Stack::new(StackRole::Dragon)).build();
        let flower_stack = state.new_entity().with_position(Point2::new(614.0, 20.0)).with_stack(Stack::new(StackRole::Flower)).build();
        state.new_entity().with_position(Point2::new(805.0, 20.0)).with_stack(Stack::new(StackRole::Target)).build();
        state.new_entity().with_position(Point2::new(957.0, 20.0)).with_stack(Stack::new(StackRole::Target)).build();
        state.new_entity().with_position(Point2::new(1109.0, 20.0)).with_stack(Stack::new(StackRole::Target)).build();
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
            game_stacks: vec!(a, b, c, d, e, f, g, h),
        };

        game.animate_shuffle();

        game
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

        //thread_rng().shuffle(&mut cards);

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
            let ani = Animation { target_pos, target_stack, start_delay: 0.0, time_left: 0.1 * i };
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

            let ani = Animation {target_pos, target_stack: Some(target_stack), start_delay, time_left: 0.2};
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
}