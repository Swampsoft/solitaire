
use cards;
use cards::{Card, Suite};
use ggez::*;
use ggez::graphics::*;

use bbox::BoundingBox;
use resources::Resources;
use rules::StackRules;

#[derive(Debug)]
pub struct CardStack {
    pos: Point2,
    rel: Vector2,
    bbox: BoundingBox,
    cards: Vec<Card>,
    rules: StackRules
}

impl CardStack {
    pub fn new_target(x: i32, y: i32) -> CardStack {
        let (x, y) = (x as f32, y as f32);
        CardStack {
            pos: Point2::new(x, y),
            rel: Vector2::new(0.1, -0.25),
            bbox: BoundingBox::new(x, x + cards::WIDTH, y, y + cards::HEIGHT),
            cards: Vec::new(),
            rules: StackRules::Target,
        }
    }

    pub fn new_dragon(x: i32, y: i32) -> CardStack {
        let (x, y) = (x as f32, y as f32);
        CardStack {
            pos: Point2::new(x, y),
            rel: Vector2::new(0.1, -0.25),
            bbox: BoundingBox::new(x, x + cards::WIDTH, y, y + cards::HEIGHT),
            cards: Vec::new(),
            rules: StackRules::Dragon,
        }
    }

    pub fn new_solitaire(x: i32, y: i32) -> CardStack {
        let (x, y) = (x as f32, y as f32);
        CardStack {
            pos: Point2::new(x, y),
            rel: Vector2::new(0.0, 32.0),
            bbox: BoundingBox::new(x, x + cards::WIDTH, y, y + cards::HEIGHT),
            cards: Vec::new(),
            rules: StackRules::Solitaire,
        }
    }

    pub fn new_rose(x: i32, y: i32) -> CardStack {
        let (x, y) = (x as f32, y as f32);
        CardStack {
            pos: Point2::new(x, y),
            rel: Vector2::new(0.1, -0.25),
            bbox: BoundingBox::new(x, x + cards::WIDTH, y, y + cards::HEIGHT),
            cards: Vec::new(),
            rules: StackRules::Flower,
        }
    }

    pub fn new_buffer(x: i32, y: i32) -> CardStack {
        let (x, y) = (x as f32, y as f32);
        CardStack {
            pos: Point2::new(x, y),
            rel: Vector2::new(0.0, 0.0),
            bbox: BoundingBox::new(x, x + cards::WIDTH, y, y + cards::HEIGHT),
            cards: Vec::new(),
            rules: StackRules::Dragging,
        }
    }

    pub fn clear(&mut self) {
        self.cards.clear();
        self.update_bounds();
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn set_pos(&mut self, pos: Point2) {
        self.pos = pos;
        self.update_bounds();
    }

    pub fn calc_card_pos(&self, i: usize) -> Point2 {
        self.pos + self.rel * i as f32
    }

    pub fn calc_new_pos(&self) -> Point2 {
        self.calc_card_pos(self.cards.len())
    }

    pub fn push_card(&mut self, mut card: Card) {
        card.set_pos(self.calc_new_pos());
        self.bbox.merge(&card.get_bounds());
        self.cards.push(card);
    }

    pub fn top_card(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn top_suite(&self) -> Option<&Suite> {
        self.cards.last().map(|c|c.suite())
    }

    pub fn push(&mut self, stack: CardStack) {
        for card in stack.cards {
            self.push_card(card);
        }
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn update_bounds(&mut self) {
        self.bbox = BoundingBox::new(self.pos.x, self.pos.x + cards::WIDTH,
                                     self.pos.y, self.pos.y + cards::HEIGHT);
        for card in &self.cards {
            self.bbox.merge(&card.get_bounds());
        }
    }

    pub fn move_pos(&mut self, dx: f32, dy: f32) {
        self.pos.x += dx;
        self.pos.y += dy;

        for card in &mut self.cards {
            card.move_pos(dx, dy);
        }

        self.bbox.topleft.x += dx;
        self.bbox.topleft.y += dy;
        self.bbox.bottomright.x += dx;
        self.bbox.bottomright.y += dy;
    }

    pub fn start_drag(&mut self, x: f32, y: f32) -> Option<CardStack> {
        if !self.bbox.is_hit(x, y) {
            return None
        }

        let mut idx = None;
        for (i, card) in self.cards.iter().enumerate().rev() {
            if card.is_hit(x, y) {
                idx = Some(i);
                break
            }
        }

        let idx = match idx {
            Some(idx) => idx,
            None => return None,
        };

        if !self.rules.accept_drag(&self.cards[idx..]) {
            return None
        }

        let cards: Vec<_> = self.cards.drain(idx..).collect();
        let mut ds = CardStack {
            pos: cards[0].get_pos(),
            rel: self.rel,
            bbox: BoundingBox::new_empty(),
            cards,
            rules: StackRules::Dragging,
        };
        ds.update_bounds();
        self.update_bounds();
        return Some(ds)
    }

    pub fn accept_drop(&self, other: &CardStack) -> bool {
        if self.bbox.intersects(&other.bbox) {
            self.rules.accept_drop(self.top_suite(),
                                   other.cards.first().expect("Who's dragging an empty card stack around???").suite(),
                                   other.cards.len())
        } else {
            false
        }
    }

    pub fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        for card in &self.cards {
            card.draw(ctx, res)?;
        }
        Ok(())
    }


}