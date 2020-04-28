use std::time;
use std::env;
use specs::prelude::*;
use crate::debug::FPS;
use ggez::graphics::Mesh;
use ggez::graphics::Drawable;
use ggez::graphics::Text;

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

#[derive(Clone)]
pub struct Rect(pub ggez::graphics::Rect);

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect(ggez::graphics::Rect::new(x, y, width, height))
    }
}

#[derive(Clone)]
pub struct RectColor(pub ggez::graphics::Color);

impl RectColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RectColor(ggez::graphics::Color::from_rgba(r, g, b, a))
    }
}

impl Component for Rect {
    type Storage = VecStorage<Self>;
}

impl Component for RectColor {
    type Storage = VecStorage<Self>;
}

pub struct Renderer;

impl<'a> Renderer {
    pub fn new(_ctx: &mut ggez::Context) -> Self {
        Renderer
    }

    pub fn run(&mut self, ctx: &mut ggez::Context, world: &'a mut World) {
        world.exec(|(rect, rect_color, fps): (ReadStorage<'a, Rect>, ReadStorage<'a, RectColor>, ReadStorage<'a, FPS>)| {
            ggez::graphics::clear(ctx, ggez::graphics::Color::from_rgb(0, 0, 0));
            for (r, c) in (&rect, &rect_color).join() {
                let mut drawable_rect = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()), r.0.clone(), c.0.clone()).unwrap();
                drawable_rect.draw(ctx, ggez::graphics::DrawParam::new());
            }
            for (f, r) in (&fps, &rect).join() {
                let mut text = Text::new(f.0.to_string());
                text.set_bounds([r.0.w, r.0.h], ggez::graphics::Align::Center);
                let mut draw_params = ggez::graphics::DrawParam::new().dest([r.0.x, r.0.y]);
                text.draw(ctx, draw_params);
            }
            ggez::graphics::present(ctx);
        });
    }
}

