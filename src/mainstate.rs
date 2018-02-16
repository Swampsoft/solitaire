
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
    game_running: bool,
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

        let dragon_stacks = vec!(0, 1, 2);
        let target_stacks = vec!(3, 4, 5);
        let solitaire_stacks: Vec<usize> = (6..14).collect();
        let flower_stack = 14;

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
            game_running: false,
        };
        Ok(s)
    }

    pub fn new_game(&mut self) {

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

        for (card, s) in cards.drain(..).zip(self.solitaire_stacks.iter().cycle()) {
            self.stacks[*s].push_card(card);
        }

        self.game_running = true;
        self.dirty = true;
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

        self.game_running = self.solitaire_stacks
            .iter()
            .map(|i|self.stacks[*i].top_card())
            .any(|tc|tc.is_some());

        if !self.resources.music.playing() {
            self.resources.music.set_volume(0.5);
            self.resources.music.play()?;
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

        if !self.game_running {
            let text = self.resources.get_text(ctx, "Click anywhere to start a new game.")?;
            text.draw(ctx, Point2::new(640.0 - text.width() as f32 / 2.0,
                                       403.0 - text.height() as f32 / 2.0), 0.0)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: i32, y: i32) {
        if !self.game_running {
            self.new_game();
            return
        }

        for (i, stack) in self.stacks.iter_mut().enumerate() {
            if let Some(s) = stack.start_drag(x as f32, y as f32) {
                self.dragsource = i;
                self.dragging = Some(s);
                self.resources.pickup_sound.play().unwrap();
                return
            }
        }
        for button in &self.buttons {
            if button.accept_click(x as f32, y as f32) {
                let t = self.find_dragon_target(button.color()).unwrap();
                for i in self.dragon_and_solitaire_stacks() {
                    if let Some(&Suite::Dragon(color)) = self.stacks[i].top_card().map(|c|c.suite()) {
                        if color == button.color() {
                            let mut card = self.stacks[i].pop().unwrap();
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
                    self.resources.place_sound.play().unwrap();
                    return
                }
            }
            self.resources.place_sound.play().unwrap();
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