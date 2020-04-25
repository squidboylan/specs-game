use specs::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time;

mod game;
use game::GameState;
mod renderer;
mod debug;

const FRAMERATE: u32 = 60;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().expect("sdl2 init failed");
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let mut event_pump = sdl_context.event_pump()?;

    let mut level = game::level::Level::new();
    let mut debug = debug::Debug::new(&mut level.world);
    let mut renderer = renderer::Renderer::new(&sdl_context, &ttf_context);

    'running: loop {
        let mut prev_time = time::Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => level.input_handler(event),
            }
        }

        level.run();
        debug.run(&mut level.world);
        renderer.run(&mut level.world);
        level.world.maintain();
        let mut curr_time = time::Instant::now();
        if time::Duration::new(0, 1_000_000_000u32 / FRAMERATE) > curr_time.duration_since(prev_time) {
            ::std::thread::sleep(time::Duration::new(0, 1_000_000_000u32 / FRAMERATE) - curr_time.duration_since(prev_time));
        }
    }
    Ok(())
}
