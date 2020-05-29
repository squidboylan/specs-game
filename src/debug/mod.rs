use crate::components::*;
use crate::renderer::SCREEN_HEIGHT;
use crate::renderer::SCREEN_WIDTH;
use specs::prelude::*;
use std::time;

pub struct Debug {
    last_frame: time::Instant,
}

impl<'a> Debug {
    pub fn new(world: &mut World) -> Self {
        world.register::<FPS>();
        world.register::<Text>();

        let width = 50.0;
        let height = 50.0;

        world
            .create_entity()
            .with(FPS)
            .with(Text {
                text: String::new(),
                location: (SCREEN_WIDTH - width, SCREEN_HEIGHT),
                scale: 1.0,
            })
            .with(Rect::new(
                SCREEN_WIDTH - width,
                SCREEN_HEIGHT - height,
                width,
                height,
            ))
            .with(RectColor::new(0.0, 0.0, 1.0, 1.0))
            .build();

        let last_frame = time::Instant::now();
        Debug { last_frame }
    }

    pub fn run(&mut self, world: &'a mut World) {
        world.exec(|(fps_flag, mut fps_string): (ReadStorage<'a, FPS>, WriteStorage<'a, Text>)| {
            let now = time::Instant::now();
            let frame_duration = now.duration_since(self.last_frame);
            let fps_num = 1_000_000_000 / frame_duration.subsec_nanos();
            for (_, s) in (&fps_flag, &mut fps_string).join() {
                s.text = fps_num.to_string();
            }
            self.last_frame = now;
        });
    }
}
