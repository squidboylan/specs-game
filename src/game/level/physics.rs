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
            rect.0.set_x(rect.0.x() + vel.x as i32);
            rect.0.set_y(rect.0.y() + vel.y as i32);
        }
    }
}
