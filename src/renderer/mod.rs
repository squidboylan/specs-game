use crate::components::*;
use crate::game::map::Map;
use glutin::WindowedContext;
use specs::prelude::*;
use std::collections::HashMap;
use std::mem;

use gl::types::*;

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

mod shader;
mod font;

type Texture = GLuint;
type Vbo = GLuint;
type Vao = GLuint;

#[repr(C)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
struct ColorRect {
    position: (f32, f32, f32, f32),
    color: (f32, f32, f32, f32),
    size: (f32, f32, f32),
    rotation: f32,
}

#[repr(C)]
struct TextureRect {
    position: (f32, f32, f32, f32),
    tile_position: (f32, f32, f32, f32),
    size: (f32, f32, f32),
    rotation: f32,
    tile_dimensions: (f32, f32),
    pad: (f32, f32),
}

pub struct Renderer {
    rect_shader: shader::Program,
    texture_shader: shader::Program,
    text_shader: shader::Program,
    mesh_vbo: Vbo,
    rects_vao: Vao,
    rects_vbo: Vbo,
    texture_rects_vao: Vao,
    texture_rects_vbo: Vbo,
    texture_rects_data: Vec<TextureRect>,
    texture_handles: HashMap<String, Texture>,
    text_rects_vao: Vao,
    text_rects_vbo: Vbo,
    font: font::Font,
}

#[repr(C)]
struct Character {
    location: (f32, f32, f32, f32),
    dimensions: (f32, f32),
    pad: (f32, f32),
}

impl<'b> Renderer {
    pub fn new() -> Self {
        let mut rect_shader = shader::Program::new(&include_str!("shaders/rect_color.vert"), &include_str!("shaders/rect_color.frag"));
        let mut texture_shader = shader::Program::new(&include_str!("shaders/texture.vert"), &include_str!("shaders/texture.frag"));
        let mut text_shader = shader::Program::new(&include_str!("shaders/text.vert"), &include_str!("shaders/text.frag"));

        let texture_handles = HashMap::new();

        let vertices: [Vertex; 6] = [
            Vertex{ x: 0.5, y: 0.5, z: 0.0 },
            Vertex{ x: 0.5, y: -0.5, z: 0.0 },
            Vertex{ x: -0.5, y: -0.5, z: 0.0 },
            Vertex{ x: -0.5, y: -0.5, z: 0.0 },
            Vertex{ x: -0.5, y: 0.5, z: 0.0 },
            Vertex{ x: 0.5, y: 0.5, z: 0.0 },
        ];

        let mut rects_data = Vec::new();
        let mut texture_rects_data = Vec::new();
        rects_data.push(ColorRect {
            position: (16.0, 16.0, 0.0, 0.0),
            color: (1.0, 0.0, 0.0, 1.0),
            size: (32.0, 32.0, 0.0),
            rotation: 0.0,
        });

        let mut mesh_vbo = 0;
        let mut rects_vao = 0;
        let mut rects_vbo = 0;

        let mut text_rects_vao = 0;
        let mut text_rects_vbo = 0;
        unsafe {
            // We probably dont need these things, at least not for now, but i'll keep the stuff
            // here and commented out just in case.
            /*
            // Enable backface culling
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CW);
            // Enable Depth Testing
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            */

            // Alpha stuff
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable( gl::BLEND );

            // Setup our rect data in the GPU
            gl::GenVertexArrays(1, &mut rects_vao);
            gl::GenBuffers(1, &mut mesh_vbo);
            gl::GenBuffers(1, &mut rects_vbo);

            gl::BindVertexArray(rects_vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, mesh_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
                mem::transmute(&vertices[0]),
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0 as i32, 0 as *const GLvoid);
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, rects_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (rects_data.len() * mem::size_of::<ColorRect>()) as GLsizeiptr,
                mem::transmute(&rects_data[0]),
                gl::STREAM_DRAW,
            );

            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<ColorRect>() as i32, (0 * mem::size_of::<f32>()) as *const GLvoid);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, mem::size_of::<ColorRect>() as i32, (4 * mem::size_of::<f32>()) as *const GLvoid);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, mem::size_of::<ColorRect>() as i32, (8 * mem::size_of::<f32>()) as *const GLvoid);
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(4, 1, gl::FLOAT, gl::FALSE, mem::size_of::<ColorRect>() as i32, (11 * mem::size_of::<f32>()) as *const GLvoid);
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribDivisor(0, 0);
            gl::VertexAttribDivisor(1, 1);
            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::BindVertexArray(0);

            // Setup our text data in the GPU
            gl::GenVertexArrays(1, &mut text_rects_vao);
            gl::GenBuffers(1, &mut text_rects_vbo);

            gl::BindVertexArray(text_rects_vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, mesh_vbo);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0 as i32, 0 as *const GLvoid);
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, text_rects_vbo);

            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<Character>() as i32, 0 as *const GLvoid);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, mem::size_of::<Character>() as i32, (4 * mem::size_of::<f32>()) as *const GLvoid);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribDivisor(0, 0);
            gl::VertexAttribDivisor(1, 1);
            gl::VertexAttribDivisor(2, 1);
            gl::BindVertexArray(0);

        }

        let font = font::Font::new(std::path::Path::new("resources/OpenSans-Regular.ttf"));

        Renderer {
            rect_shader,
            texture_shader,
            text_shader,
            mesh_vbo,
            rects_vao,
            rects_vbo,
            texture_rects_vao: 0,
            texture_rects_vbo: 0,
            texture_handles,
            texture_rects_data,
            text_rects_vao,
            text_rects_vbo,
            font,
        }
    }

    pub fn run(&mut self, ctx: &mut WindowedContext<glutin::PossiblyCurrent>, world: &'b mut World) {
        self.draw_background(ctx, world);
        let mut rects_data = Vec::new();
        world.exec(
            |(rect, rect_color, rotation): (
                ReadStorage<'b, Rect>,
                ReadStorage<'b, RectColor>,
                ReadStorage<'b, Rotation>,
            )| {
                // Render our color rects
                self.rect_shader.enable();

                for (r, c, rot) in (&rect, &rect_color, rotation.maybe()).join() {
                    let rot = if let Some(x) = rot {
                        x.0
                    } else {
                        0.0
                    };
                    let center = r.get_center();
                    rects_data.push(ColorRect {
                        position: (center.0, center.1, 0.0, 1.0),
                        color: (c.r, c.g, c.b, c.a),
                        size: (r.w, r.h, 0.0),
                        rotation: rot,
                    });
                }
                unsafe {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.rects_vbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (rects_data.len() * mem::size_of::<ColorRect>()) as GLsizeiptr,
                        mem::transmute(&rects_data[0]),
                        gl::STREAM_DRAW,
                    );
                    gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT | gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);

                    gl::BindVertexArray(self.rects_vao);
                    gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, rects_data.len() as i32);
                    gl::BindVertexArray(0);
                }
            },
        );
        self.draw_text(ctx, world);
    }

    pub fn draw_text(&mut self, ctx: &mut WindowedContext<glutin::PossiblyCurrent>, world: &'b mut World) {
        world.exec(
            |(rect, text): (
                ReadStorage<'b, Rect>,
                ReadStorage<'b, Text>,
            )| {
                // Render text
                self.text_shader.enable();


                unsafe {
                    gl::BindVertexArray(self.text_rects_vao);
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.text_rects_vbo);
                    gl::Uniform3f(gl::GetUniformLocation(self.text_shader.program, "color".as_ptr() as *const GLchar), 1.0, 0.0, 0.0);
                    for (r, t) in (&rect, &text).join() {
                        let mut curr_x = t.location.0;
                        let mut curr_y = t.location.1;
                        for character in t.text.as_bytes() {
                            let glyph = &self.font.glyphs[*character as usize];
                            let bitmap = &self.font.bitmaps[*character as usize];
                            gl::BindTexture(gl::TEXTURE_2D, self.font.font_textures[*character as usize]);
                            let x = curr_x + glyph.bitmap_left() as f32;
                            let y = curr_y + 24.0 - glyph.bitmap_top() as f32;
                            let loc = (x, y, 1.0, 1.0);
                            let tmp = [Character {
                                location: loc,
                                dimensions: (bitmap.width() as f32, bitmap.rows() as f32),
                                pad: (0.0, 0.0)
                            }];
                            gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (tmp.len() * mem::size_of::<Character>()) as GLsizeiptr,
                                mem::transmute(&tmp[0]),
                                gl::STREAM_DRAW,
                            );
                            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, 1 as i32);
                            let a = glyph.advance();
                            curr_x += a.x as f32 / 64.0;
                            curr_y += a.y as f32 / 64.0;
                        }
                    }
                    gl::BindVertexArray(0);
                    gl::BindTexture(gl::TEXTURE_2D, 0);
                }
            },
        );
    }

    pub fn prepare_map(&mut self, world: &'b mut World) {
        let map = world.fetch_mut::<crate::game::map::Map>();
        if self.texture_rects_data.len() == 0 {
            let image = image::open(&map.image).unwrap().to_rgba();
            unsafe {
                gl::GenVertexArrays(1, &mut self.texture_rects_vao);
                gl::GenBuffers(1, &mut self.texture_rects_vbo);

                gl::BindVertexArray(self.texture_rects_vao);

                gl::BindBuffer(gl::ARRAY_BUFFER, self.mesh_vbo);
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0 as i32, 0 as *const GLvoid);
                gl::EnableVertexAttribArray(0);

                gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_rects_vbo);

                gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<TextureRect>() as i32, (0 * mem::size_of::<f32>()) as *const GLvoid);
                gl::EnableVertexAttribArray(1);
                gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, mem::size_of::<TextureRect>() as i32, (4 * mem::size_of::<f32>()) as *const GLvoid);
                gl::EnableVertexAttribArray(2);
                gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, mem::size_of::<TextureRect>() as i32, (8 * mem::size_of::<f32>()) as *const GLvoid);
                gl::EnableVertexAttribArray(3);
                gl::VertexAttribPointer(4, 1, gl::FLOAT, gl::FALSE, mem::size_of::<TextureRect>() as i32, (11 * mem::size_of::<f32>()) as *const GLvoid);
                gl::EnableVertexAttribArray(4);
                gl::VertexAttribPointer(5, 2, gl::FLOAT, gl::FALSE, mem::size_of::<TextureRect>() as i32, (12 * mem::size_of::<f32>()) as *const GLvoid);
                gl::EnableVertexAttribArray(5);
                gl::VertexAttribDivisor(0, 0);
                gl::VertexAttribDivisor(1, 1);
                gl::VertexAttribDivisor(2, 1);
                gl::VertexAttribDivisor(3, 1);
                gl::VertexAttribDivisor(4, 1);
                gl::VertexAttribDivisor(5, 1);
                gl::BindVertexArray(0);

                // Setup our texture stuff
                let mut texture = 0;
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
                println!("foo");
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, image.width() as GLsizei, image.height() as GLsizei,
                    0, gl::RGBA, gl::UNSIGNED_BYTE, image.into_raw().as_ptr() as *const GLvoid);

                self.texture_handles.insert(map.image.clone(), texture);
            }
        }
        self.texture_rects_data.clear();

        for layer in &map.layers {
            for tile in &layer.map_tiles {
                let image_tile = map.get_tile(tile.tile_num);
                if image_tile.is_none() {
                    continue;
                }
                let image_tile = image_tile.unwrap().rect;

                self.texture_rects_data.push(
                    TextureRect {
                        position: (tile.loc[0], tile.loc[1], 0.0, 1.0),
                        tile_position: (image_tile.x as f32, image_tile.y as f32, 0.0, 1.0),
                        size: (32.0, 32.0, 0.0),
                        rotation: 0.0,
                        tile_dimensions: (image_tile.w as f32, image_tile.h as f32),
                        pad: (0.0, 0.0),
                    }
                );
            }
        }

        unsafe {
            gl::BindVertexArray(self.texture_rects_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_rects_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.texture_rects_data.len() * mem::size_of::<TextureRect>()) as GLsizeiptr,
                mem::transmute(&self.texture_rects_data[0]),
                gl::STREAM_DRAW,
            );
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_background(&mut self, ctx: &mut WindowedContext<glutin::PossiblyCurrent>, world: &'b mut World) {
        if world.try_fetch_mut::<crate::game::map::Map>().is_none() {
            return;
        }
        let image = world.fetch::<crate::game::map::Map>().image.clone();
        /*
        let mut texture = self.texture_handles.get(&image);
        if texture.is_none() {
            self.prepare_map(world);
        }
        (*/
        self.prepare_map(world);
        let map = world.try_fetch_mut::<crate::game::map::Map>().unwrap();
        let mut texture = self.texture_handles.get(&map.image).unwrap();
        //println!("drawing {} tiles", self.texture_rects_data.len());
        self.texture_shader.enable();
        unsafe {
            gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT | gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);

            gl::BindTexture(gl::TEXTURE_2D, *texture);
            gl::BindVertexArray(self.texture_rects_vao);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, self.texture_rects_data.len() as i32);
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
