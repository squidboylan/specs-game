use specs::prelude::*;
use std::time;
use crate::renderer::Rect;
use crate::renderer::SCREEN_WIDTH;
use crate::renderer::SCREEN_HEIGHT;

pub struct FPS(pub u32);

impl Component for FPS {
    type Storage = VecStorage<Self>;
}

struct FPSCounter {
    last_frame: time::Instant,
}

impl<'a> System<'a> for FPSCounter {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = WriteStorage<'a, FPS>;

    fn run(&mut self, mut fps_string: Self::SystemData) {
        let now = time::Instant::now();
        let frame_duration = now.duration_since(self.last_frame);
        let fps_num = 1_000_000_000/frame_duration.subsec_nanos();
        for f in (&mut fps_string).join() {
            f.0 = fps_num;
        }
        self.last_frame = now;
    }
}

pub struct Debug<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Debug<'a, 'b> {
    pub fn new(world: &mut World) -> Self {
        world.register::<FPS>();

        let width = 100;
        let height = 100;

        world.create_entity().with(FPS(0)).with(Rect::new(SCREEN_WIDTH as i32 - width, SCREEN_HEIGHT as i32 - height, width as u32, height as u32)).build();

        let dispatcher = DispatcherBuilder::new()
            .with(FPSCounter{ last_frame: time::Instant::now() }, "fpscounter", &[])
            .build();

        Debug{ dispatcher }
    }

    pub fn run(&mut self, world: &mut World) {
        self.dispatcher.dispatch(world);
    }
}
