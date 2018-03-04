use std::collections::HashMap;
use std::slice;

use ggez::{Context, GameResult};
use ggez::graphics::Point2;

mod animation_systems;
mod input_systems;
mod render_systems;
mod rule_systems;
mod rules;
pub mod types;

use resources::Resources;

use self::animation_systems::*;
use self::render_systems::*;
use self::rule_systems::*;
use self::types::*;

type Component<T> = Vec<Option<T>>;

#[derive(Default)]
pub struct GameState {
    entities: Vec<Entity>,
    ent_lookup: HashMap<Entity, usize>,

    stacks: Component<Stack>,
    positions: Component<Point2>,
    zorder: Component<f32>,
    buttons: Component<Button>,
    animations: Component<Animation>,

    next_id: usize,

    busy: bool,
    dirty: bool,
    render_queue: RenderQueue,

    drag_lock: Option<(Entity, Entity)>
}

impl GameState {
    pub fn new() -> GameState {
        let mut state = GameState::default();
        state
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    pub fn remove_entity(&mut self, id: Entity) {
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

    pub fn clear(&mut self) {
        self.entities.clear();
        self.ent_lookup.clear();

        self.stacks.clear();
        self.positions.clear();
        self.zorder.clear();
        self.buttons.clear();
        self.animations.clear();

        self.next_id = 0;

        self.busy = false;
        self.dirty = false;

        self.drag_lock = None;
    }

    pub fn iter(&self) -> slice::Iter<Entity> {
        self.entities.iter()
    }

    pub fn get_stack(&self, id: Entity) -> Option<&Stack> {
        let idx = self.ent_lookup[&id];
        self.stacks[idx].as_ref()
    }

    pub fn get_stack_mut(&mut self, id: Entity) -> Option<&mut Stack> {
        let idx = self.ent_lookup[&id];
        self.stacks[idx].as_mut()
    }

    pub fn get_position(&self, id: Entity) -> Option<&Point2> {
        let idx = self.ent_lookup[&id];
        self.positions[idx].as_ref()
    }

    pub fn get_position_mut(&mut self, id: Entity) -> Option<&mut Point2> {
        let idx = self.ent_lookup[&id];
        self.positions[idx].as_mut()
    }

    pub fn take_position(&mut self, id: Entity) -> Option<Point2> {
        let idx = self.ent_lookup[&id];
        self.positions[idx].take()
    }

    pub fn get_zorder(&self, id: Entity) -> Option<&f32> {
        let idx = self.ent_lookup[&id];
        self.zorder[idx].as_ref()
    }

    pub fn get_button(&self, id: Entity) -> Option<&Button> {
        let idx = self.ent_lookup[&id];
        self.buttons[idx].as_ref()
    }

    pub fn get_animation(&self, id: Entity) -> Option<&Animation> {
        let idx = self.ent_lookup[&id];
        self.animations[idx].as_ref()
    }

    pub fn busy(&self) -> bool {
        self.busy
    }

    pub fn run_update(&mut self, dt: f32, res: &mut Resources) -> bool {
        self.busy = false;
        self.busy |= self.animation_update_system(dt, res);
        self.button_update_system();
        self.auto_move_system();
        self.busy
    }

    pub fn run_render(&mut self, ctx: &mut Context, res: &mut Resources) -> GameResult<()> {
        self.render_queue.background_render_system(ctx, res)?;
        self.render_queue.button_render_system(ctx, res, &self.positions, &self.buttons)?;
        self.render_queue.stack_render_system(ctx, res, &self.positions, &self.stacks, &self.zorder)?;
        self.render_queue.render(ctx, res)?;
        Ok(())
    }

    pub fn handle_mouse_button_down(&mut self, x: i32, y: i32, res: &Resources) {
        let pos = Point2::new(x as f32, y as f32);
        self.begin_drag_system(pos, res);
        self.button_click_system(pos);
    }

    pub fn handle_mouse_button_up(&mut self, x: i32, y: i32, res: &Resources) {
        let pos = Point2::new(x as f32, y as f32);
        self.done_drag_system(res);
    }

    pub fn handle_mouse_move(&mut self, xrel: i32, yrel: i32) {
        let dpos = Vector2::new(xrel as f32, yrel as f32);
        self.do_drag_system(dpos);
    }

    pub fn animate(&mut self, card: Suite, pos: Point2, z: f32, ani: Animation) {
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

pub struct EntityBuilder<'a> {
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

    pub fn build(self) -> Entity {
        let id = Entity::new(self.state.next_id);
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

    pub fn with_stack(mut self, value: Stack) -> EntityBuilder<'a> {
        self.stack = Some(value);
        self
    }

    pub fn with_position(mut self, pos: Point2) -> EntityBuilder<'a> {
        self.position = Some(pos);
        self
    }

    pub fn with_zorder(mut self, z: f32) -> EntityBuilder<'a> {
        self.zorder = Some(z);
        self
    }

    pub fn with_button(mut self, value: Button) -> EntityBuilder<'a> {
        self.button = Some(value);
        self
    }

    pub fn with_animation(mut self, value: Animation) -> EntityBuilder<'a> {
        self.animation = Some(value);
        self
    }
}