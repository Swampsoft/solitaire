
use ggez::*;
use ggez::graphics::*;
use ggez::event::*;

use cards::{Card, Color, Suite};
use cardstack::CardStack;
use resources::Resources;

use rand;

pub struct MainState {
    resources: Resources,
    dragging: Option<CardStack>,
    dragsource: usize,
    stacks: Vec<CardStack>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        /*let mut cards = Vec::with_capacity(5);
        cards.resize(5, Card::new());
        for c in &mut cards {
            c.set_pos(rand::random::<f32>() * 1000.0, rand::random::<f32>() * 600.0)
        }*/

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
        };

        for _ in 0..1 {
            stacks[0].add_card(Card::new(Suite::Dragon(Color::Red)));
            stacks[1].add_card(Card::new(Suite::Dragon(Color::Green)));
            stacks[2].add_card(Card::new(Suite::Dragon(Color::White)));
        }
        for _ in 0..1 {
            stacks[3].add_card(Card::new(Suite::Number(4, Color::Red)));
            stacks[4].add_card(Card::new(Suite::Number(5, Color::Green)));
            stacks[5].add_card(Card::new(Suite::Number(6, Color::White)));
        }
        for _ in 0..1 {
            stacks[6].add_card(Card::new(Suite::Number(7, Color::Red)));
            stacks[7].add_card(Card::new(Suite::Number(8, Color::Green)));
            stacks[8].add_card(Card::new(Suite::Number(9, Color::White)));
            stacks[9].add_card(Card::new(Suite::Number(1, Color::Red)));
            stacks[10].add_card(Card::new(Suite::Number(2, Color::Red)));
            stacks[11].add_card(Card::new(Suite::Number(3, Color::Green)));
            stacks[12].add_card(Card::new(Suite::Number(4, Color::Red)));
            stacks[13].add_card(Card::new(Suite::Flower));
        }

        let s = MainState {
            resources: Resources::new(ctx)?,
            dragging: None,
            dragsource: 0,
            stacks
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        //println!("FPS: {}", timer::get_fps(ctx));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0));
        graphics::draw(ctx, &self.resources.table_image, Point2::new(0.0, 0.0), 0.0)?;

        for stack in &self.stacks {
            stack.draw(ctx, &self.resources)?;
        }

        if let Some(ref stack) = self.dragging {
            stack.draw(ctx, &self.resources)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        for (i, stack) in self.stacks.iter_mut().enumerate() {
            if let Some(s) = stack.start_drag(x as f32, y as f32) {
                self.dragsource = i;
                self.dragging = Some(s);
                break
            }
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: i32, _y: i32) {
        if let Some(dstack) = self.dragging.take() {
            for (i, stack) in self.stacks.iter_mut().enumerate() {
                if i == self.dragsource {
                    continue
                }
                if stack.accept(&dstack) {
                    stack.push(dstack);
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