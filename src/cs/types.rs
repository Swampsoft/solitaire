use std::slice;

pub use ggez::graphics::{Point2, Vector2};

pub use button::ButtonState;
pub use cards::Color;

const MAX_STACK_SIZE: usize = 14;

pub struct Button {
    pub color: Color,
    pub state: ButtonState,
}

impl Button {
    pub fn new(color: Color) -> Button {
        Button {
            color,
            state: ButtonState::Up,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Suite {
    FaceDown,
    Flower,
    Dragon(Color),
    Number(u8, Color),
}

#[derive(Debug)]
pub enum StackRole {
    Dragon,
    DragonLocked,
    Flower,
    Target,
    Generic,
    Animation,
}

pub struct Stack {
    pub cards: Vec<Suite>,
    pub role: StackRole,
}

impl Stack {
    pub fn new(role: StackRole) -> Stack {
        Stack {
            cards: Vec::new(),
            role
        }
    }

    pub fn cards_iter(&self) -> slice::Iter<Suite> {
        self.cards.iter()
    }

    pub fn push_card(&mut self, card: Suite) {
        self.cards.push(card);
    }

    pub fn pop_card(&mut self) -> Option<Suite> {
        self.cards.pop()
    }

    pub fn get_stackshift(&self) -> Vector2 {
        match self.role {
            StackRole::Dragon => Vector2::new(0.1, -0.25),
            StackRole::DragonLocked => Vector2::new(0.1, -0.25),
            StackRole::Flower => Vector2::new(0.1, -0.25),
            StackRole::Target => Vector2::new(0.1, -0.25),
            StackRole::Generic => Vector2::new(0.0, 32.0),
            StackRole::Animation => Vector2::new(0.0, 0.0),
        }
    }
}

pub struct Animation {
    pub time_left: f32,
    pub target_pos: Point2,
    pub target_stack: Option<usize>,
}
