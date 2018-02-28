use std::collections::HashMap;

use ggez::{Context, GameResult};
use ggez::graphics::Point2;

mod animation_systems;
mod render_systems;
mod types;

use resources::Resources;

use self::animation_systems::*;
use self::render_systems::*;
use self::types::*;

type Component<T> = Vec<Option<T>>;

#[derive(Default)]
pub struct GameState {
    entities: Vec<usize>,
    ent_lookup: HashMap<usize, usize>,

    stacks: Component<Stack>,
    positions: Component<Point2>,
    zorder: Component<f32>,
    buttons: Component<Button>,
    animations: Component<Animation>,

    next_id: usize,
}

impl GameState {
    pub fn new() -> GameState {
        let mut state = GameState::default();

        state.new_entity().with_position(Point2::new(533.0, 54.0)).with_button(Button::new(Color::Red)).build();
        state.new_entity().with_position(Point2::new(533.0, 137.0)).with_button(Button::new(Color::Green)).build();
        state.new_entity().with_position(Point2::new(533.0, 220.0)).with_button(Button::new(Color::White)).build();

        state.new_entity().with_position(Point2::new(45.0, 20.0)).with_stack(Stack::new(StackRole::Dragon)).build();
        state.new_entity().with_position(Point2::new(197.0, 20.0)).with_stack(Stack::new(StackRole::Dragon)).build();
        state.new_entity().with_position(Point2::new(349.0, 20.0)).with_stack(Stack::new(StackRole::Dragon)).build();
        let f = state.new_entity().with_position(Point2::new(614.0, 20.0)).with_stack(Stack::new(StackRole::Flower)).build();
        state.new_entity().with_position(Point2::new(805.0, 20.0)).with_stack(Stack::new(StackRole::Target)).build();
        state.new_entity().with_position(Point2::new(957.0, 20.0)).with_stack(Stack::new(StackRole::Target)).build();
        state.new_entity().with_position(Point2::new(1109.0, 20.0)).with_stack(Stack::new(StackRole::Target)).build();
        state.new_entity().with_position(Point2::new(45.0, 283.0)).with_stack(Stack::new(StackRole::Generic)).build();
        state.new_entity().with_position(Point2::new(197.0, 283.0)).with_stack(Stack::new(StackRole::Generic)).build();
        state.new_entity().with_position(Point2::new(349.0, 283.0)).with_stack(Stack::new(StackRole::Generic)).build();
        state.new_entity().with_position(Point2::new(501.0, 283.0)).with_stack(Stack::new(StackRole::Generic)).build();
        state.new_entity().with_position(Point2::new(653.0, 283.0)).with_stack(Stack::new(StackRole::Generic)).build();
        state.new_entity().with_position(Point2::new(805.0, 283.0)).with_stack(Stack::new(StackRole::Generic)).build();
        state.new_entity().with_position(Point2::new(957.0, 283.0)).with_stack(Stack::new(StackRole::Generic)).build();
        state.new_entity().with_position(Point2::new(1109.0, 283.0)).with_stack(Stack::new(StackRole::Generic)).build();

        let target_stack = Some(f);
        let stack_pos = state.positions[state.ent_lookup[&f]].unwrap();
        let shift = state.stacks[state.ent_lookup[&f]].as_ref().unwrap().get_stackshift();
        for n in 0..40 {
            let i = 1.0 + 0.1 * (n as f32);
            let start_pos = stack_pos - shift * i * (stack_pos.y + render_systems::CARD_HEIGHT) / shift.y;
            let target_pos = stack_pos + shift * n as f32;
            state.animate(Suite::FaceDown, start_pos, 100.0 + n as f32, Animation { target_pos, target_stack, time_left: 1.0 * i });
        }

        state
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    pub fn remove_entity(&mut self, id: usize) {
        let idx = self.ent_lookup[&id];

        // last entity takes the place of id, so we need to update the lookup
        self.ent_lookup.insert(*self.entities.last().unwrap(), idx);
        self.ent_lookup.remove(&id);

        self.stacks.swap_remove(idx);
        self.positions.swap_remove(idx);
        self.zorder.swap_remove(idx);
        self.buttons.swap_remove(idx);
        self.animations.swap_remove(idx);

        self.entities.swap_remove(idx);
    }

    pub fn run_update(&mut self, dt: f32) -> bool {
        let mut busy = false;
        busy |= self.animation_update_system(dt);
        return busy;
    }

    pub fn run_render(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        let mut rq = RenderQueue::new();
        rq.background_render_system(ctx, res)?;
        rq.button_render_system(ctx, res, &self.positions, &self.buttons)?;
        rq.stack_render_system(ctx, res, &self.positions, &self.stacks, &self.zorder)?;
        rq.render(ctx, res)?;
        Ok(())
    }

    fn animate(&mut self, card: Suite, pos: Point2, z: f32, ani: Animation) {
        let mut stack = Stack::new(StackRole::Animation);
        stack.push_card(card);

        self.new_entity()
            .with_position(pos)
            .with_zorder(z)
            .with_animation(ani)
            .with_stack(stack)
            .build();
    }
}

struct EntityBuilder<'a> {
    state: &'a mut GameState,
    stack: Option<Stack>,
    position: Option<Point2>,
    zorder: Option<f32>,
    button: Option<Button>,
    animation: Option<Animation>,
}

impl<'a> EntityBuilder<'a> {
    fn new(state: &'a mut GameState) -> EntityBuilder<'a> {
        EntityBuilder {
            state,
            stack: None,
            position: None,
            zorder: Some(0.0),
            button: None,
            animation: None,
        }
    }

    fn build(self) -> usize {
        let id = self.state.next_id;
        let idx = self.state.entities.len();
        self.state.entities.push(id);
        self.state.ent_lookup.insert(id, idx);
        self.state.next_id += 1;

        self.state.stacks.push(self.stack);
        self.state.positions.push(self.position);
        self.state.zorder.push(self.zorder);
        self.state.buttons.push(self.button);
        self.state.animations.push(self.animation);
        id
    }

    fn with_stack(mut self, value: Stack) -> EntityBuilder<'a> {
        self.stack = Some(value);
        self
    }

    fn with_position(mut self, pos: Point2) -> EntityBuilder<'a> {
        self.position = Some(pos);
        self
    }

    fn with_zorder(mut self, z: f32) -> EntityBuilder<'a> {
        self.zorder = Some(z);
        self
    }

    fn with_button(mut self, value: Button) -> EntityBuilder<'a> {
        self.button = Some(value);
        self
    }

    fn with_animation(mut self, value: Animation) -> EntityBuilder<'a> {
        self.animation = Some(value);
        self
    }
}