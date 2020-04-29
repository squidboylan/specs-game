use specs::prelude::*;
use crate::renderer::Rect;
use crate::renderer::RectColor;

pub struct Physics;

pub struct Vel{
    pub x: f32,
    pub y: f32,
}

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Rect>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut rect, vel): Self::SystemData) {
        for (rect, vel) in (&mut rect, &vel).join() {
            rect.x = rect.x + vel.x;
            rect.y = rect.y + vel.y;
        }
    }
}
