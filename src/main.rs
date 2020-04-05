use specs::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod game;
mod renderer;
mod debug;

fn main() -> Result<(), String> {
    let mut world = World::new();

    let sdl_context = sdl2::init().expect("sdl2 init failed");
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let mut event_pump = sdl_context.event_pump()?;

    let mut game = game::Game::new(&mut world);
    let mut debug = debug::Debug::new(&mut world);
    let mut renderer = renderer::Renderer::new(&sdl_context, &ttf_context);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        game.run(&mut world);
        debug.run(&mut world);
        renderer.run(&mut world);
        world.maintain();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
    Ok(())
}
