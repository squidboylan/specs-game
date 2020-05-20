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

#[repr(C)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
struct RenderableRect {
    position: (f32, f32, f32, f32),
    color: (f32, f32, f32, f32),
    size: f32,
    rotation: f32,
    pad: [f32; 2],
}

pub struct Renderer {
    rect_shader: shader::Program,
    mesh_vbo: GLuint,
    rects_vao: GLuint,
    rects_vbo: GLuint,
}

impl<'b> Renderer {
    pub fn new() -> Self {
        let mut rect_shader = shader::Program::new(&include_str!("shader.vert"), &include_str!("shader.frag"));

        let vertices: [Vertex; 6] = [
            Vertex{ x: 0.5, y: 0.5, z: 0.0 },
            Vertex{ x: 0.5, y: -0.5, z: 0.0 },
            Vertex{ x: -0.5, y: -0.5, z: 0.0 },
            Vertex{ x: -0.5, y: -0.5, z: 0.0 },
            Vertex{ x: -0.5, y: 0.5, z: 0.0 },
            Vertex{ x: 0.5, y: 0.5, z: 0.0 },
        ];

        let mut rects_data = Vec::new();
        rects_data.push(RenderableRect {
            position: (16.0, -16.0, 0.0, 0.0),
            color: (1.0, 0.0, 0.0, 1.0),
            size: 32.0,
            rotation: 0.0,
            pad: [0.0, 0.0],
        });

        let mut mesh_vbo = 0;
        let mut rects_vao = 0;
        let mut rects_vbo = 0;
        unsafe {
            // Enable backface culling
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CW);
            // Enable Depth Testing
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);

            // Setup our particle data in the GPU
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
                (rects_data.len() * mem::size_of::<RenderableRect>()) as GLsizeiptr,
                mem::transmute(&rects_data[0]),
                gl::STREAM_DRAW,
            );

            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<RenderableRect>() as i32, (0 * mem::size_of::<f32>()) as *const GLvoid);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, mem::size_of::<RenderableRect>() as i32, (4 * mem::size_of::<f32>()) as *const GLvoid);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(3, 1, gl::FLOAT, gl::FALSE, mem::size_of::<RenderableRect>() as i32, (8 * mem::size_of::<f32>()) as *const GLvoid);
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(4, 1, gl::FLOAT, gl::FALSE, mem::size_of::<RenderableRect>() as i32, (9 * mem::size_of::<f32>()) as *const GLvoid);
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribDivisor(0, 0);
            gl::VertexAttribDivisor(1, 1);
            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::BindVertexArray(0);
        }

        Renderer {
            rect_shader,
            mesh_vbo,
            rects_vao,
            rects_vbo,
        }
    }

    pub fn run(&mut self, ctx: &mut WindowedContext<glutin::PossiblyCurrent>, world: &'b mut World) {
        /*
        world.exec(
            |(rect, rect_color, _text, rotation): (
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
        */
        self.rect_shader.enable();
        unsafe {
            gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT | gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);

            gl::BindVertexArray(self.rects_vao);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, 1 as i32);
            gl::BindVertexArray(0);
        }

    }

    pub fn draw_background(&mut self, ctx: &mut WindowedContext<glutin::PossiblyCurrent>, world: &'b mut World) {
        return;
    }
}
