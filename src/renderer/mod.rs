use std::time;
use specs::prelude::*;
use sdl2::pixels::Color;
use crate::debug::FPS;

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
    pub fn new(sdl_context: &sdl2::Sdl, ttf_context: &'b sdl2::ttf::Sdl2TtfContext) -> Self {
        let video_subsystem = sdl_context.video().expect("Couldnt get sdl video context");

        let window = video_subsystem.window("rust-sdl2 demo: Video", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).expect("Couldnt initialize an sdl opengl window");

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("Couldnt get an sdl canvas");
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let font = ttf_context.load_font("fonts/OpenSans-Regular.ttf", 128).unwrap();

        let dispatcher = DispatcherBuilder::new()
            .with_thread_local(RenderSystem{canvas, font}).build();

        Renderer{ dispatcher }
    }

    pub fn run(&mut self, world: &mut World) {
        self.dispatcher.dispatch(world);
    }
}

struct RenderSystem<'a> {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    font: sdl2::ttf::Font<'a, 'static>,
}

impl<'a, 'b> System<'a> for RenderSystem<'b> {
    type SystemData = (ReadStorage<'a, Rect>, ReadStorage<'a, FPS>);

    fn run(&mut self, (rect, fps): Self::SystemData) {
        let texture_creator = self.canvas.texture_creator();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        for i in (&rect).join() {
            self.canvas.fill_rect(i.0).unwrap();
        }
        for f in (&fps).join() {
            let surface = self.font.render(&f.0.to_string())
                    .blended(Color::RGBA(0, 255, 0, 255)).map_err(|e| e.to_string()).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string()).unwrap();

            self.canvas.copy(&texture, None, Some(sdl2::rect::Rect::new(100, 100, 600, 600))).unwrap();
        }
        self.canvas.present();
    }
}
