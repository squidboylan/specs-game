use crate::components::*;
use ggez::graphics;
use ggez::graphics::Drawable;
use specs::prelude::*;

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

pub struct Renderer {
    font: graphics::Font,
}

impl<'a> Renderer {
    pub fn new(_ctx: &mut ggez::Context) -> Self {
        let font = graphics::Font::default();
        Renderer { font }
    }

    pub fn run(&mut self, ctx: &mut ggez::Context, world: &'a mut World) {
        world.exec(
            |(rect, rect_color, text, rotation): (
                ReadStorage<'a, Rect>,
                ReadStorage<'a, RectColor>,
                ReadStorage<'a, Text>,
                ReadStorage<'a, Rotation>,
            )| {
                graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
                for (r, c, rot) in (&rect, &rect_color, rotation.maybe()).join() {
                    let drawable_rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::Fill(graphics::FillOptions::default()),
                        *r.clone(),
                        *c.clone(),
                    )
                    .unwrap();
                    let mut draw_params = graphics::DrawParam::new();
                    if let Some(x) = rot {
                        draw_params = draw_params.rotation(x.0).offset([r.x + r.w/2.0, r.y + r.h/2.0]);
                    }
                    drawable_rect
                        .draw(ctx, draw_params)
                        .expect("Failed to draw a rectangle");
                }
                for (t, r) in (&text, &rect).join() {
                    let mut drawable_text = graphics::Text::new(t.text.clone());
                    drawable_text.set_font(self.font, t.scale);
                    drawable_text.set_bounds([r.w, r.h], graphics::Align::Center);
                    let draw_params = graphics::DrawParam::new().dest([r.x, r.y]);
                    drawable_text
                        .draw(ctx, draw_params)
                        .expect("Failed to draw text");
                }
                graphics::present(ctx).expect("Failed to present the graphics");
            },
        );
    }
}
