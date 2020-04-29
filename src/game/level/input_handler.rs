use specs::prelude::*;
use super::physics::Vel;
use crate::game::input::Input;
use crate::game::*;
use crate::renderer::Rect;

pub struct InputHandler;

impl<'a> System<'a> for InputHandler {
    type SystemData = (WriteStorage<'a, Rect>, WriteStorage<'a, Vel>, ReadStorage<'a, Player>, ReadStorage<'a, Cursor>, Read<'a, Input>);

    fn run(&mut self, (mut rect, mut vel, player, cursor, input): Self::SystemData) {
        let velocity = 2.0;
        for (v, _) in (&mut vel, &player).join() {
            if input.keyboard.W == true {
                v.y = -1.0 * velocity;
            }
            if input.keyboard.A == true {
                v.x = -1.0 * velocity;
            }
            if input.keyboard.S == true {
                v.y = velocity;
            }
            if input.keyboard.D == true {
                v.x = velocity;
            }

            if input.keyboard.W == false && input.keyboard.S == false {
                v.y = 0.0;
            }
            if input.keyboard.A == false && input.keyboard.D == false {
                v.x = 0.0;
            }
        }

        for (r, _) in (&mut rect, &cursor).join() {
            r.x = input.mouse.x - r.w/2.0;
            r.y = input.mouse.y - r.h/2.0;
        }
    }
}
