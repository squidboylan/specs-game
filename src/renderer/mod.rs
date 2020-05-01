use specs::prelude::*;
use crate::components::*;
use ggez::graphics;
use ggez::graphics::Drawable;

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

pub struct Renderer{
    font: graphics::Font,
}

impl<'a> Renderer {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let font = graphics::Font::default();
        Renderer{font}
    }

    pub fn run(&mut self, ctx: &mut ggez::Context, world: &'a mut World) {
        world.exec(|(rect, rect_color, text): (ReadStorage<'a, Rect>, ReadStorage<'a, RectColor>, ReadStorage<'a, Text>)| {
            graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
            for (r, c) in (&rect, &rect_color).join() {
                let drawable_rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::Fill(graphics::FillOptions::default()), *r.clone(), *c.clone()).unwrap();
                drawable_rect.draw(ctx, graphics::DrawParam::new()).expect("Failed to draw a rectangle");
            }
            for (t, r) in (&text, &rect).join() {
                let mut drawable_text = graphics::Text::new(t.text.clone());
                drawable_text.set_font(self.font, t.scale);
                drawable_text.set_bounds([r.w, r.h], graphics::Align::Center);
                let draw_params = graphics::DrawParam::new().dest([r.x, r.y]);
                drawable_text.draw(ctx, draw_params).expect("Failed to draw text");
            }
            graphics::present(ctx).expect("Failed to present the graphics");
        });
    }
}

