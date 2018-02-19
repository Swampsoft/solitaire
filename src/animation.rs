
use std::cmp;
use std::collections::BinaryHeap;
use std::time;

use ggez::{Context, GameResult};
use ggez::graphics::{Point2, Vector2};

use cards::Card;
use resources::{Resources, Sounds};

enum Path {
    Linear{dest: Point2, vel: Vector2}
}

impl Path {
    fn new_linear(a: Point2, b: Point2, dur: f32) -> Path {
        Path::Linear{
            dest: b,
            vel: (b - a) / dur
        }
    }

    fn pos(&self, t: f32) -> Point2 {
        match *self {
            Path::Linear{ref dest, ref vel} => dest - vel * t
        }
    }
}

pub struct Animation {
    card: Card,

    dest_stack: usize,

    path: Path,
    t_start: time::Duration,
    t_stop: time::Duration,

    sound_start: Sounds,
    sound_stop: Sounds,
}

impl Animation {
    pub fn new(card: Card, dest: Point2, t_start: time::Duration, t_stop: time::Duration,
               dest_stack: usize, sound_start: Sounds, sound_stop: Sounds) -> Animation {
        let dt = t_stop - t_start;
        let dur = dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9;
        let path = Path::new_linear(card.get_pos(), dest, dur);
        Animation {
            card,
            t_start,
            t_stop,
            dest_stack,
            path,
            sound_start,
            sound_stop,
        }
    }

    fn update(&mut self, t_now: time::Duration) -> bool {
        match self.t_stop.checked_sub(t_now) {
            Some(dt) => {
                let t = dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9;
                self.card.set_pos(self.path.pos(t));
                true
            },
            None => {
                self.card.set_pos(self.path.pos(0.0));
                false
            }
        }
    }
}

pub struct AnimationHandler {
    active_animations: Vec<Animation>,
    pending_animations: Vec<Animation>,
}

impl AnimationHandler {
    pub fn new() -> AnimationHandler {
        AnimationHandler {
            active_animations: Vec::new(),
            pending_animations: Vec::new(),
        }
    }

    pub fn busy(&self) -> bool {
        self.active_animations.len() > 0 || self.pending_animations.len() > 0
    }

    pub fn add(&mut self, animation: Animation) {
        self.pending_animations.push(animation);
    }

    pub fn update(&mut self, t_now: time::Duration, res: &mut Resources) -> Vec<(Card, usize)> {
        let mut start_animations = Vec::new();
        for (i, anim) in self.pending_animations.iter().enumerate() {
            if anim.t_start <= t_now {
                start_animations.push(i);
            }
        }

        for a in start_animations.drain(..).rev() {
            let anim = self.pending_animations.remove(a);
            res.play_sound(anim.sound_start);
            self.active_animations.push(anim);
        }

        let mut done_animations = Vec::new();
        for (i, animation) in self.active_animations.iter_mut().enumerate() {
            if !animation.update(t_now) {
                done_animations.push(i);
            }
        }

        let mut result = Vec::new();
        for a in done_animations.drain(..).rev() {
            let dead_anim = self.active_animations.swap_remove(a);
            res.play_sound(dead_anim.sound_stop);
            result.push((dead_anim.card, dead_anim.dest_stack))
        }
        result
    }

    pub fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        for animation in self.pending_animations.iter().rev().chain(self.active_animations.iter()) {
            animation.card.draw(ctx, res)?;
        }
        Ok(())
    }
}

// =========================
//       boiler plate

impl cmp::Ord for Animation {
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        // Note the reversed order of the comparison.
        // This is because `BinaryHeap` returns the maximum element, but we want the
        // earliest start time.
        rhs.t_start.cmp(&self.t_start)
    }
}

impl cmp::PartialOrd for Animation {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(rhs))
    }

}

impl cmp::PartialEq for Animation {
    fn eq(&self, rhs: &Self) -> bool {
        self.t_start == rhs.t_start
    }
}

impl cmp::Eq for Animation { }