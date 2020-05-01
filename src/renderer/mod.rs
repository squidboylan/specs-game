



use specs::prelude::*;
use crate::components::*;
use ggez::graphics::Mesh;
use ggez::graphics::Drawable;
use ggez::graphics::Text;

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;
pub struct Renderer;

impl<'a> Renderer {
    pub fn new(_ctx: &mut ggez::Context) -> Self {
        Renderer
    }

    pub fn run(&mut self, ctx: &mut ggez::Context, world: &'a mut World) {
        world.exec(|(rect, rect_color, fps): (ReadStorage<'a, Rect>, ReadStorage<'a, RectColor>, ReadStorage<'a, FPS>)| {
            ggez::graphics::clear(ctx, ggez::graphics::Color::from_rgb(0, 0, 0));
            for (r, c) in (&rect, &rect_color).join() {
                let drawable_rect = Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()), *r.clone(), *c.clone()).unwrap();
                drawable_rect.draw(ctx, ggez::graphics::DrawParam::new()).expect("Failed to draw a rectangle");
            }
            for (f, r) in (&fps, &rect).join() {
                let mut text = Text::new(f.0.to_string());
                text.set_bounds([r.w, r.h], ggez::graphics::Align::Center);
                let draw_params = ggez::graphics::DrawParam::new().dest([r.x, r.y]);
                text.draw(ctx, draw_params).expect("Failed to draw text");
            }
            ggez::graphics::present(ctx).expect("Failed to present the graphics");
        });
    }
}

