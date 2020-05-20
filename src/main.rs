use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ ContextBuilder, GlRequest, Api };
use glutin::dpi::LogicalSize;
use glutin::dpi::PhysicalSize;

use gl::types::*;

use std::path;
use std::env;

mod components;
mod debug;
mod game;
mod renderer;
mod systems;

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("rogue-like?")
        .with_maximized(true)
        .with_inner_size(PhysicalSize::new(renderer::SCREEN_WIDTH as f64, renderer::SCREEN_HEIGHT as f64));


    let context = ContextBuilder::new()
        .with_vsync(true)
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 3)))
        .with_multisampling(0)
        .build_windowed(window, &event_loop)
        .unwrap();

    let mut windowed_context = unsafe { context.make_current().unwrap() };
    windowed_context.window().set_cursor_visible(false);
    windowed_context.window().set_cursor_grab(true);

    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    let mut game = game::Game::new();

    let mut window_open = true;

    while window_open {
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        windowed_context.resize(physical_size);
                        unsafe {
                            gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                        }
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::KeyboardInput { input, ..} => {
                        game.key_event(input);
                    }
                    WindowEvent::CursorMoved { position, ..} => {
                        game.mouse_movement(position);
                    }
                    WindowEvent::MouseInput { state, button, ..} => {
                        game.mouse_button_down_event(button, state);
                    },
                    WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
                        println!("scale_factor: {:?}", scale_factor);
                        println!("inner_size: {:?}", new_inner_size);
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    println!("Drawing");
                    game.update();
                    unsafe {
                        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    }
                    game.draw(&mut windowed_context);
                    windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
