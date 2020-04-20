use specs::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use super::physics::Vel;

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Input {
    W: bool,
    A: bool,
    S: bool,
    D: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            W: false,
            A: false,
            S: false,
            D: false,
        }
    }

}

pub fn update_input(world: &mut World, event: Event) {
    let mut input = world.fetch_mut::<Input>();
    match event {
        Event::KeyUp { keycode: Some(Keycode::W), ..} => input.W = false,
        Event::KeyUp { keycode: Some(Keycode::A), ..} => input.A = false,
        Event::KeyUp { keycode: Some(Keycode::S), ..} => input.S = false,
        Event::KeyUp { keycode: Some(Keycode::D), ..} => input.D = false,
        Event::KeyDown { keycode: Some(Keycode::W), ..} => input.W = true,
        Event::KeyDown { keycode: Some(Keycode::A), ..} => input.A = true,
        Event::KeyDown { keycode: Some(Keycode::S), ..} => input.S = true,
        Event::KeyDown { keycode: Some(Keycode::D), ..} => input.D = true,
        _ => {},
    }
}

pub struct InputHandler;

impl<'a> System<'a> for InputHandler {
    type SystemData = (WriteStorage<'a, Vel>, ReadStorage<'a, Player>, Read<'a, Input>);

    fn run(&mut self, (mut vel, player, input): Self::SystemData) {
        let velocity = 2.0;
        for (v, _) in (&mut vel, &player).join() {
            if input.W == true {
                v.y = -1.0 * velocity;
            }
            if input.A == true {
                v.x = -1.0 * velocity;
            }
            if input.S == true {
                v.y = velocity;
            }
            if input.D == true {
                v.x = velocity;
            }

            if input.W == false && input.S == false {
                v.y = 0.0;
            }
            if input.A == false && input.D == false {
                v.x = 0.0;
            }
        }
    }
}
