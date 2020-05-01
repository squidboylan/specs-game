use specs::prelude::*;
use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Rect>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut rect, vel): Self::SystemData) {
        for (rect, vel) in (&mut rect, &vel).join() {
            rect.x += vel.x;
            rect.y += vel.y;
        }
    }
}
