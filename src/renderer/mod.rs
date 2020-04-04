use specs::prelude::*;
use sdl2::pixels::Color;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

#[derive(Clone)]
pub struct Rect(pub sdl2::rect::Rect);

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rect(sdl2::rect::Rect::new(x, y, width, height))
    }
}

pub struct Renderer<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn new(world: &mut World, sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().expect("Couldnt get sdl video context");

        let window = video_subsystem.window("rust-sdl2 demo: Video", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).expect("Couldnt initialize an sdl opengl window");

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("Couldnt get an sdl canvas");
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let mut dispatcher = DispatcherBuilder::new()
            .with_thread_local(RenderSystem{canvas}).build();

        Renderer{ dispatcher }
    }

    pub fn run(&mut self, world: &mut World) {
        self.dispatcher.dispatch(world);
    }
}

struct RenderSystem {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = ReadStorage<'a, Rect>;

    fn run(&mut self, rect: Self::SystemData) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        for (i) in (&rect).join() {
            self.canvas.fill_rect(i.0).unwrap();
        }
        self.canvas.present();
    }
}
