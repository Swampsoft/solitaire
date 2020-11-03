use std::slice;

use ggez::graphics;

pub use resources::Sounds;

pub const CARD_WIDTH: f32 = 123.0;
pub const CARD_HEIGHT: f32 = 233.0;

pub const BUTTON_RADIUS: f32 = 30.0;
pub const BUTTON_RADIUS_SQUARED: f32 = BUTTON_RADIUS * BUTTON_RADIUS;

pub type Point2 = ggez::nalgebra::Point2<f32>;
pub type Vector2 = ggez::nalgebra::Vector2<f32>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Entity(usize);

impl Entity {
    pub fn new(id: usize) -> Entity {
        Entity(id)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    Red,
    Green,
    White,
}

impl Color {
    pub fn to_font_color(&self) -> graphics::Color {
        match *self {
            Color::Red => graphics::Color::new(0.7, 0.2, 0.1, 1.0),
            Color::Green => graphics::Color::new(0.1, 0.4, 0.3, 1.0),
            Color::White => graphics::Color::new(0.1, 0.1, 0.1, 1.0),
        }
    }
    pub fn to_icon_color(&self) -> graphics::Color {
        match *self {
            Color::Red => graphics::Color::new(1.0, 1.0, 1.0, 1.0),
            Color::Green => graphics::Color::new(0.1, 0.4, 0.3, 1.0),
            Color::White => graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ButtonState {
    Active,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Button {
    pub color: Color,
    pub state: ButtonState,
    pub stacks: Option<(Entity, [Entity; 4])>,
}

impl Button {
    pub fn new(color: Color) -> Button {
        Button {
            color,
            state: ButtonState::Up,
            stacks: None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Suite {
    FaceDown,
    Flower,
    Dragon(Color),
    Number(u8, Color),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StackRole {
    Dragon,
    Flower,
    Target,
    Sorting,
    Generic,
    Animation,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Stack {
    pub cards: Vec<Suite>,
    pub role: StackRole,
}

impl Stack {
    pub fn new(role: StackRole) -> Stack {
        Stack {
            cards: Vec::new(),
            role,
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn iter(&self) -> slice::Iter<Suite> {
        self.cards.iter()
    }

    pub fn top(&self) -> Option<Suite> {
        self.cards.last().map(|s| *s)
    }

    pub fn push_card(&mut self, card: Suite) {
        self.cards.push(card);
    }

    pub fn pop_card(&mut self) -> Option<Suite> {
        self.cards.pop()
    }

    pub fn extend(&mut self, other: Stack) {
        self.cards.extend(other.cards);
    }

    pub fn peek(&self, idx: usize) -> Suite {
        self.cards[idx]
    }

    pub fn get_stackshift(&self) -> Vector2 {
        match self.role {
            StackRole::Dragon => Vector2::new(0.1, -0.25),
            //StackRole::DragonLocked => Vector2::new(0.1, -0.25),
            StackRole::Flower => Vector2::new(0.1, -0.25),
            StackRole::Target => Vector2::new(0.1, -0.25),
            StackRole::Sorting => Vector2::new(0.0, 32.0),
            StackRole::Generic => Vector2::new(0.0, 32.0),
            StackRole::Animation => Vector2::new(0.0, 0.0),
        }
    }

    pub fn split(&mut self, at: usize) -> Stack {
        Stack {
            cards: self.cards.split_off(at),
            role: StackRole::Generic,
        }
    }
}

pub struct Animation {
    pub start_delay: f32,
    pub time_left: f32,
    pub target_pos: Point2,
    pub target_stack: Option<Entity>,

    pub sound_start: Sounds,
    pub sound_stop: Sounds,
}
