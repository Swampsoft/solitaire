
use cards;
use cards::Card;
use ggez::*;
use ggez::graphics::*;

use bbox::BoundingBox;
use resources::Resources;


#[derive(Debug)]
enum StackRules {
    Dragging,
    Target,
    Dragon,
    Rose,
    Solitaire
}

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
            rel: Vector2::new(0.5, -1.0),
            bbox: BoundingBox::new(x, x + cards::WIDTH, y, y + cards::HEIGHT),
            cards: Vec::new(),
            rules: StackRules::Target,
        }
    }

    pub fn new_dragon(x: i32, y: i32) -> CardStack {
        let (x, y) = (x as f32, y as f32);
        CardStack {
            pos: Point2::new(x, y),
            rel: Vector2::new(0.5, -1.0),
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
            rel: Vector2::new(0.5, -1.0),
            bbox: BoundingBox::new(x, x + cards::WIDTH, y, y + cards::HEIGHT),
            cards: Vec::new(),
            rules: StackRules::Rose,
        }
    }

    pub fn add_card(&mut self, mut card: Card) {
        card.set_pos(self.pos + self.rel * self.cards.len() as f32);
        self.bbox.merge(&card.get_bounds());
        self.cards.push(card);
    }

    pub fn push(&mut self, stack: CardStack) {
        for card in stack.cards {
            self.add_card(card);
        }
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

        match self.rules {
            StackRules::Target |
            StackRules::Rose => return None,
            StackRules::Dragon => {
                match self.cards.last() {
                    None => return None,
                    Some(card) => if !card.is_faceup() { return None }
                }
                let card = self.cards.pop().unwrap();
                let ds = CardStack {
                    pos: card.get_pos(),
                    rel: self.rel,
                    bbox: card.get_bounds(),
                    cards: vec!(card),
                    rules: StackRules::Dragging,
                };
                self.update_bounds();
                return Some(ds)
            }
            StackRules::Solitaire => {
                let mut idx = None;
                for (i, card) in self.cards.iter().enumerate().rev() {
                    if card.is_hit(x, y) {
                        idx = Some(i);
                        break
                    }
                }
                if idx.is_none() {
                    return None
                }
                let idx = idx.unwrap();
                let mut ds = CardStack {
                    pos: self.cards[idx].get_pos(),
                    rel: self.rel,
                    bbox: BoundingBox::new_empty(),
                    cards: self.cards.drain(idx..).collect(),
                    rules: StackRules::Dragging,
                };
                ds.update_bounds();
                self.update_bounds();
                return Some(ds)
            },
            StackRules::Dragging => panic!("Attempting to drag from a dragged stack")
        }
    }

    pub fn accept(&self, other: &CardStack) -> bool {
        // TODO: implement rules
        self.bbox.is_touching(&other.bbox)
    }

    pub fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        for card in &self.cards {
            card.draw(ctx, res)?;
        }
        Ok(())
    }


}