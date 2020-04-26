use specs::prelude::*;
use crate::renderer::Rect;
use crate::renderer::RectColor;
use crate::game::input::Input;
use crate::game::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod input_handler;
pub mod physics;

pub use input_handler::*;
pub use physics::*;

struct Creator {
    y: i32,
}

impl Creator {
    pub fn new(y: i32) -> Self {
        Creator{y}
    }
}

impl<'a> System<'a> for Creator {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, lazy): Self::SystemData) {
        let i = entities.create();
        let r = Rect::new(0, self.y, 10, 10);
        let c = RectColor::new(0, self.y as u8, 0, 255);
        lazy.insert(i, r);
        lazy.insert(i, c);
        lazy.insert(i, Vel{x: 2.0, y: 0.0});
        self.y+=1;
    }
}


pub struct Level<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    pub world: World
}

impl<'a, 'b> GameState for Level<'a, 'b> {
    fn input_handler(&mut self, event: Event) {
        let mut input = self.world.fetch_mut::<Input>();
        match event {
            Event::KeyUp { keycode: Some(Keycode::W), ..} => input.keyboard.W = false,
            Event::KeyUp { keycode: Some(Keycode::A), ..} => input.keyboard.A = false,
            Event::KeyUp { keycode: Some(Keycode::S), ..} => input.keyboard.S = false,
            Event::KeyUp { keycode: Some(Keycode::D), ..} => input.keyboard.D = false,
            Event::KeyDown { keycode: Some(Keycode::W), ..} => input.keyboard.W = true,
            Event::KeyDown { keycode: Some(Keycode::A), ..} => input.keyboard.A = true,
            Event::KeyDown { keycode: Some(Keycode::S), ..} => input.keyboard.S = true,
            Event::KeyDown { keycode: Some(Keycode::D), ..} => input.keyboard.D = true,
            Event::MouseMotion { x: x, y: y, ..} => { input.mouse.x = x; input.mouse.y = y },
            _ => println!("{:?}", event),
        }
    }

    fn get_mut_world(&mut self) -> &mut World {
        &mut self.world
    }

    fn run(&mut self) -> Option<StateTransition> {
        self.dispatcher.dispatch(&self.world);
        None
    }
}

impl<'a, 'b> Level<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();

        world.insert(Input::new());

        world.register::<Rect>();
        world.register::<RectColor>();
        world.register::<Vel>();
        world.register::<Player>();
        world.register::<Cursor>();

        let rect = Rect::new(0, 1, 5, 5);
        let color = RectColor::new(255, 0, 0, 255);
        let cursor_color = RectColor::new(255, 255, 255, 255);

        world.create_entity()
            .with(Player)
            .with(Vel{x: 0.0, y: 0.0})
            .with(rect.clone())
            .with(color.clone())
            .build();
        world.create_entity()
            .with(Cursor)
            .with(Vel{x: 0.0, y: 0.0})
            .with(rect.clone())
            .with(cursor_color.clone())
            .build();
        world.create_entity()
            .with(Vel{x: 1.0, y: 0.0})
            .with(rect.clone())
            .with(color.clone())
            .build();
        world.create_entity()
            .with(Vel{x: 0.0, y: 2.0})
            .with(rect.clone())
            .with(color.clone())
            .build();

        let dispatcher = DispatcherBuilder::new()
            .with(InputHandler, "input_handler", &[])
            .with(Physics, "physics", &["input_handler"])
            .with(Creator::new(20), "creator", &[])
            .build();

        Level{ dispatcher, world }
    }

    pub fn from_world(world: World) -> Self {
        let dispatcher = DispatcherBuilder::new()
            .with(InputHandler, "input_handler", &[])
            .with(Physics, "physics", &["input_handler"])
            .with(Creator::new(20), "creator", &[])
            .build();

        Level{ dispatcher, world }
    }
}
