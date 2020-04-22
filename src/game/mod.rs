use specs::prelude::*;
use crate::renderer::Rect;
use crate::renderer::RectColor;

mod physics;
mod input;

pub use input::*;
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


pub struct Game<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(world: &mut World) -> Self {
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

        Game{ dispatcher }
    }

    pub fn run(&mut self, world: &mut World) {
        self.dispatcher.dispatch(world);
    }
}
