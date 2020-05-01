use specs::prelude::*;
use crate::game::input::Input;
use crate::game::*;


pub struct InputHandler;

impl<'a> System<'a> for InputHandler {
    type SystemData = (WriteStorage<'a, Rect>, WriteStorage<'a, Vel>, ReadStorage<'a, Player>, ReadStorage<'a, Cursor>, Read<'a, Input>);

    fn run(&mut self, (mut rect, mut vel, player, cursor, input): Self::SystemData) {
        let velocity = 2.0;
        for (v, _) in (&mut vel, &player).join() {
            if input.keyboard.w {
                v.y = -1.0 * velocity;
            }
            if input.keyboard.a {
                v.x = -1.0 * velocity;
            }
            if input.keyboard.s {
                v.y = velocity;
            }
            if input.keyboard.d {
                v.x = velocity;
            }

            if !input.keyboard.w && !input.keyboard.s {
                v.y = 0.0;
            }
            if !input.keyboard.a && !input.keyboard.d {
                v.x = 0.0;
            }
        }

        for (r, _) in (&mut rect, &cursor).join() {
            r.x = input.mouse.x - r.w/2.0;
            r.y = input.mouse.y - r.h/2.0;
        }
    }
}
