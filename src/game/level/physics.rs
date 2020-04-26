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
            rect.0.left = rect.0.left + vel.x as i32;
            rect.0.top = rect.0.top + vel.y as i32;
        }
    }
}
