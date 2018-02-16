
use std::iter;
use std::slice;

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;

use rand::{thread_rng, Rng};

use button;
use cards::{Card, Color, Suite};
use cardstack::CardStack;
use resources::Resources;
use rules;

pub struct MainState {
    resources: Resources,
    dragging: Option<CardStack>,
    dragsource: usize,
    pub stacks: Vec<CardStack>,
    pub buttons: Vec<button::Button>,
    pub dragon_stacks: Vec<usize>,
    pub target_stacks: Vec<usize>,
    pub solitaire_stacks: Vec<usize>,
    pub flower_stack: usize,
    dirty: bool,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        /*let mut cards = Vec::with_capacity(5);
        cards.resize(5, Card::new());
        for c in &mut cards {
            c.set_pos(rand::random::<f32>() * 1000.0, rand::random::<f32>() * 600.0)
        }*/

        let buttons = vec!{
            button::Button::new(Color::Red, Point2::new(533.0, 54.0)),
            button::Button::new(Color::Green, Point2::new(533.0, 137.0)),
            button::Button::new(Color::White, Point2::new(533.0, 220.0)),
        };

        let mut stacks = vec!{
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

        let dragon_stacks = vec!(0, 1, 2);
        let target_stacks = vec!(3, 4, 5);
        let solitaire_stacks: Vec<usize> = (6..14).collect();
        let flower_stack = 14;

        let mut cards = Vec::with_capacity(40);

        for i in 1..10 {
            cards.push(Card::new(Suite::Number(i, Color::Red)));
            cards.push(Card::new(Suite::Number(i, Color::Green)));
            cards.push(Card::new(Suite::Number(i, Color::White)));
        }

        for i in 0..4 {
            cards.push(Card::new(Suite::Dragon(Color::Red)));
            cards.push(Card::new(Suite::Dragon(Color::Green)));
            cards.push(Card::new(Suite::Dragon(Color::White)));
        }

        cards.push(Card::new(Suite::Flower));

        thread_rng().shuffle(&mut cards);

        for (card, s) in cards.drain(..).zip(solitaire_stacks.iter().cycle()) {
            stacks[*s].push_card(card);
        }

        let s = MainState {
            resources: Resources::new(ctx)?,
            dragging: None,
            dragsource: 0,
            stacks,
            buttons,
            dragon_stacks,
            target_stacks,
            solitaire_stacks,
            flower_stack,
            dirty: true,
        };
        Ok(s)
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn find_dragon_target(&self, color: Color) -> Option<usize> {
        let mut target = None;
        for i in self.dragon_stacks.iter() {
            match self.stacks[*i].top_suite() {
                Some(&Suite::Dragon(c)) if c == color => return Some(*i),
                None => target = Some(*i),
                _ => {}
            }
        }
        target
    }

    pub fn dragon_and_solitaire_stacks(&self) -> Vec<usize> {
        self.dragon_stacks.iter().chain(&self.solitaire_stacks).map(|i|*i).collect()
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        //println!("FPS: {}", timer::get_fps(ctx));
        while self.dirty {
            self.dirty = false;
            rules::global_rules(self);  // TODO: actually we only need to call this after a card was moved
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.resources.table_image, Point2::new(0.0, 0.0), 0.0)?;

        for button in &self.buttons {
            button.draw(ctx, &self.resources)?;
        }

        for stack in &self.stacks {
            stack.draw(ctx, &self.resources)?;
        }

        if let Some(ref stack) = self.dragging {
            stack.draw(ctx, &self.resources)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: i32, y: i32) {
        for (i, stack) in self.stacks.iter_mut().enumerate() {
            if let Some(s) = stack.start_drag(x as f32, y as f32) {
                self.dragsource = i;
                self.dragging = Some(s);
                return
            }
        }
        for button in &self.buttons {
            if button.accept_click(x as f32, y as f32) {
                println!("Button {:?} accepted click", button);
                let t = self.find_dragon_target(button.color()).unwrap();
                println!("Target: {}", t);
                for i in self.dragon_and_solitaire_stacks() {
                    if let Some(&Suite::Dragon(color)) = self.stacks[i].top_card().map(|c|c.suite()) {
                        if color == button.color() {
                            let mut card = self.stacks[i].pop().unwrap();
                            println!("Moving {:?} from {} to {}", card, i, t);
                            card.set_faceup(false);
                            self.stacks[t].push_card(card);
                            self.dirty = true;
                        }
                    }
                }
            }
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        if let Some(dstack) = self.dragging.take() {
            for (i, stack) in self.stacks.iter_mut().enumerate() {
                if i == self.dragsource {
                    continue
                }
                if stack.accept(&dstack) {
                    stack.push(dstack);
                    self.dirty = true;
                    return
                }
            }
            self.stacks[self.dragsource].push(dstack);
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState,
                          _x: i32, _y: i32, xrel: i32, yrel: i32) {
        if let Some(ref mut stack) = self.dragging {
            stack.move_pos(xrel as f32, yrel as f32);
        }
    }
}