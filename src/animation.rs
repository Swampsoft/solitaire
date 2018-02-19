
use std::cmp;
use std::collections::BinaryHeap;
use std::time;

use ggez::{Context, GameResult};
use ggez::graphics::{Point2, Vector2};

use cards::Card;
use resources::Resources;

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

struct Animation {
    card: Card,

    dest_stack: Option<usize>,

    path: Path,
    t_start: time::Duration,
    t_stop: time::Duration,
}

impl Animation {
    fn new(card: Card, dest: Point2, t_start: time::Duration, t_stop: time::Duration, dest_stack: Option<usize>) -> Animation {
        let dt = t_stop - t_start;
        let dur = dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9;
        let path = Path::new_linear(card.get_pos(), dest, dur);
        Animation {
            card,
            t_start,
            t_stop,
            dest_stack,
            path,
        }
    }

    fn update(&mut self, t_now: time::Duration) {
        let dt = self.t_stop - t_now;
        let t = dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9;
        self.card.set_pos(self.path.pos(t));
    }
}

struct AnimationHandler {
    active_animations: Vec<Animation>,
    pending_animations: BinaryHeap<Animation>,
    done_animations: Vec<usize>,
}

impl AnimationHandler {
    fn new() -> AnimationHandler {
        AnimationHandler {
            active_animations: Vec::new(),
            pending_animations: BinaryHeap::new(),
            done_animations: Vec::new(),
        }
    }

    fn update(&mut self, t_now: time::Duration) {
        self.done_animations.sort_unstable();
        for a in self.done_animations.drain(..).rev() {
            self.active_animations.swap_remove(a);
        }

        loop {
            match self.pending_animations.peek() {
                None => break,
                Some(a) if a.t_start > t_now => break,
                Some(_) => {}
            }
            self.active_animations.push(self.pending_animations.pop().unwrap());
        }

        for animation in &mut self.active_animations {
            animation.update(t_now);
        }
    }

    pub fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        for animation in self.pending_animations.iter().chain(self.active_animations.iter()) {
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