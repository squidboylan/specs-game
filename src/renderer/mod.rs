use std::time;
use specs::prelude::*;
use sfml::{
    audio::{Sound, SoundBuffer},
    graphics::{
        CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
        Transformable,
    },
    system::{Clock, Time, Vector2f},
    window::{ContextSettings, Event, Key, Style},
};
use crate::debug::FPS;

pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1080;

#[derive(Clone)]
pub struct Rect(pub sfml::graphics::Rect<i32>);

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect(sfml::graphics::Rect::new(x, y, width, height))
    }
}

#[derive(Clone)]
pub struct RectColor(pub Color);

impl RectColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RectColor(Color::rgba(r, g, b, a))
    }
}

impl Component for Rect {
    type Storage = VecStorage<Self>;
}

impl Component for RectColor {
    type Storage = VecStorage<Self>;
}

pub struct Renderer {
    pub window: RenderWindow,
    font: sfml::system::SfBox<Font>,
}

impl<'a> Renderer {
    pub fn new() -> Self {
        let context_settings = ContextSettings {
            antialiasing_level: 0,
            ..Default::default()
        };
        let mut window = RenderWindow::new(
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            "SFML Pong",
            Style::CLOSE,
            &context_settings,
            );
        window.set_vertical_sync_enabled(true);
        window.set_mouse_cursor_visible(false);
        window.set_mouse_cursor_grabbed(true);
        window.set_framerate_limit(60);

        let font = Font::from_file("fonts/OpenSans-Regular.ttf").unwrap();

        Renderer{ window, font }
    }

    pub fn run(&mut self, world: &'a mut World) {
        world.exec(|(rect, rect_color, fps): (ReadStorage<'a, Rect>, ReadStorage<'a, RectColor>, ReadStorage<'a, FPS>)| {
            //let texture_creator = self.canvas.texture_creator();
            self.window.clear(Color::rgb(0, 0, 0));
            for (r, c) in (&rect, &rect_color).join() {
                let mut drawable_rect = RectangleShape::new();
                drawable_rect.set_position((r.0.left as f32, r.0.top as f32));
                drawable_rect.set_size((r.0.width as f32, r.0.height as f32));
                drawable_rect.set_fill_color(c.0);
                self.window.draw(&drawable_rect);
            }
            for (f, r) in (&fps, &rect,).join() {
                let mut text = Text::new(&f.0.to_string(), &self.font, 40);
                text.set_position((r.0.left as f32, r.0.top as f32));
                //text.set_size((r.0.width as f32, r.0.height as f32));
                text.set_fill_color(Color::rgb(255, 255, 255));
                self.window.draw(&text);
            }
            self.window.display();
        });
    }
}

