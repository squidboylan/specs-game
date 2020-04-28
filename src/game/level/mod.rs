use specs::prelude::*;
use crate::renderer::Rect;
use crate::renderer::RectColor;
use crate::game::input::Input;
use crate::game::*;
use std::mem;

pub mod input_handler;
pub mod physics;

pub use input_handler::*;
pub use physics::*;

struct Creator {
    y: f32,
}

impl Creator {
    pub fn new(y: f32) -> Self {
        Creator{y}
    }
}

impl<'a> System<'a> for Creator {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, lazy): Self::SystemData) {
        let i = entities.create();
        let r = Rect::new(0.0, self.y, 10.0, 10.0);
        let c = RectColor::new(0, self.y as u8, 0, 255);
        lazy.insert(i, r);
        lazy.insert(i, c);
        lazy.insert(i, Vel{x: 2.0, y: 0.0});
        self.y+=1.0;
    }
}


pub struct Level<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    pub world: World
}

impl<'a, 'b> GameState for Level<'a, 'b> {
    fn get_mut_world(&mut self) -> &mut World {
        &mut self.world
    }

    fn run(&mut self) -> Option<StateTransition> {
        self.dispatcher.dispatch(&self.world);
        mem::replace(&mut *self.world.fetch_mut::<Option<StateTransition>>(), None)
    }
}

impl<'a, 'b> Level<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();

        world.insert(Input::new());
        world.insert::<Option<StateTransition>>(None);

        world.register::<Rect>();
        world.register::<RectColor>();
        world.register::<Vel>();
        world.register::<Player>();
        world.register::<Cursor>();

        let rect = Rect::new(0.0, 1.0, 5.0, 5.0);
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
            .with(Creator::new(20.0), "creator", &[])
            .build();

        Level{ dispatcher, world }
    }

    pub fn from_world(world: World) -> Self {
        let dispatcher = DispatcherBuilder::new()
            .with(InputHandler, "input_handler", &[])
            .with(Physics, "physics", &["input_handler"])
            .with(Creator::new(20.0), "creator", &[])
            .build();

        Level{ dispatcher, world }
    }
}
