use crate::components::*;
use ggez::graphics;
use ggez::graphics::Drawable;
use specs::prelude::*;
use std::collections::HashMap;

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

pub struct Renderer {
    font: graphics::Font,
    image_cache: HashMap<String, graphics::Image>,
}

impl<'b> Renderer {
    pub fn new(_ctx: &mut ggez::Context) -> Self {
        let font = graphics::Font::default();
        let image_cache = HashMap::new();
        Renderer { font, image_cache }
    }

    pub fn run(&mut self, ctx: &mut ggez::Context, world: &'b mut World) {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
        self.draw_background(ctx, world);
        world.exec(
            |(rect, rect_color, text, rotation): (
                ReadStorage<'b, Rect>,
                ReadStorage<'b, RectColor>,
                ReadStorage<'b, Text>,
                ReadStorage<'b, Rotation>,
            )| {
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
    pub fn draw_background(&mut self, ctx: &mut ggez::Context, world: &'b mut World) {
        world.exec(
            |map: Option<Read<'b, tiled::Map>>| {
                if map.is_none() {
                    return;
                }
                let map = map.unwrap();
                for layer in &map.layers {
                    for (row_num, tile_row) in layer.tiles.iter().enumerate() {
                        for (col_num, tile) in tile_row.iter().enumerate() {
                            let render_data = self.get_tile_data(ctx, &map, tile.gid);
                            if render_data.is_none() {
                                continue;
                            }
                            let (rect, image) = render_data.unwrap();
                            let x = col_num as f32 * 32.0;
                            let y = row_num as f32 * 32.0;
                            let draw_params = graphics::DrawParam::new().src(rect).dest([x, y]);
                            image.draw(ctx, draw_params).expect("Failed to draw background tile");
                        }
                    }
                }
            },
        );
    }

    pub fn get_tile_data(&mut self, ctx: &mut ggez::Context, map: &tiled::Map, gid: u32) -> Option<(graphics::Rect, &graphics::Image)> {
        for ts in &map.tilesets {
            if gid >= ts.first_gid {
                if gid <= ts.first_gid + ts.tilecount.unwrap() as u32 {
                    // There could be more than one image, but im not gonna worry about that rn
                    let image = &ts.images[0];
                    let columns = (image.width as u32 - ts.margin)/(ts.tile_width + ts.spacing);
                    let rect = graphics::Rect::new(
                        (ts.margin + ((gid - ts.first_gid) % columns) * (ts.tile_width + ts.spacing)) as f32/image.width as f32,
                        (ts.margin + ((gid - ts.first_gid) / columns) * (ts.tile_height + ts.spacing)) as f32/image.height as f32,
                        ts.tile_width as f32/image.width as f32,
                        ts.tile_height as f32/image.height as f32,
                    );
                    if self.image_cache.contains_key(&image.source) {
                        return Some((rect, self.image_cache.get(&image.source).unwrap()));
                    } else {
                        let mut path = "/".to_string();
                        path.push_str(&image.source);
                        let image = self.image_cache.entry(image.source.to_string()).or_insert(graphics::Image::new(ctx, &path).unwrap());
                        return Some((rect, image));
                    }
                }
            }
        }
        None
    }
}
