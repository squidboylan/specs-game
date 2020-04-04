use specs::prelude::*;
use crate::renderer::Rect;

#[derive(Default)]
struct Y(i32);

struct Name(String);

impl Component for Name {
    type Storage = VecStorage<Self>;
}

struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

impl Component for Rect {
    type Storage = VecStorage<Self>;
}

struct Physics;

impl<'a> System<'a> for Physics {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteStorage<'a, Rect>, ReadStorage<'a, Vel>, ReadStorage<'a, Name>);

    fn run(&mut self, (mut rect, vel, _name): Self::SystemData) { for (rect, vel) in (&mut rect, &vel).join() {
            rect.0.set_x(rect.0.x() + vel.0 as i32);
        }
    }
}

struct Creator {
    y: i32,
}

impl Creator {
    pub fn new(y: i32) -> Self {
        Creator{y}
    }
}

impl<'a> System<'a> for Creator {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, lazy): Self::SystemData) {
        let i = entities.create();
        let r = Rect::new(0, self.y, 10, 10);
        lazy.insert(i, r);
        lazy.insert(i, Vel(2.0));
        self.y+=1;
    }
}


pub struct Game<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(world: &mut World) -> Self {
        world.register::<Rect>();
        world.register::<Vel>();
        world.register::<Name>();

        // An entity may or may not contain some component.
        let rect = Rect::new(0, 1, 5, 5);

        world.create_entity().with(Name("A".to_string())).with(Vel(2.0)).with(rect.clone()).build();
        world.create_entity().with(Name("B".to_string())).with(Vel(4.0)).with(rect.clone()).build();
        world.create_entity().with(Name("C".to_string())).with(Vel(1.5)).with(rect.clone()).build();

        let dispatcher = DispatcherBuilder::new()
            .with(Physics, "physics", &[])
            .with(Creator::new(20), "creator", &[])
            .build();

        Game{ dispatcher }
    }

    pub fn run(&mut self, world: &mut World) {
        self.dispatcher.dispatch(world);
    }
}
