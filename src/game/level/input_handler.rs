use specs::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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
            r.0.x = input.mouse.x - r.0.w/2;
            r.0.y = input.mouse.y - r.0.h/2;
        }
    }
}