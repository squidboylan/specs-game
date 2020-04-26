use specs::prelude::*;
use crate::renderer;
use crate::debug;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time;

mod level;
mod menu;
mod input;

const FRAMERATE: u32 = 60;

pub trait GameState {
    fn input_handler(&mut self, event: Event);

    fn get_mut_world(&mut self) -> &mut World;

    fn run(&mut self);
}

pub struct Game<'a, 'b> {
    debug: debug::Debug<'a, 'b>,
    renderer: renderer::Renderer<'a, 'b>,
    state_stack: Vec<Box<dyn GameState>>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(sdl_context: &sdl2::Sdl, ttf_context: &'b sdl2::ttf::Sdl2TtfContext) -> Self {
        //let mut level = level::Level::new();
        let mut menu = menu::Menu::new();
        let mut debug = debug::Debug::new(&mut menu.world);
        let mut renderer = renderer::Renderer::new(sdl_context, ttf_context);
        let mut state_stack: Vec<Box<dyn GameState>> = Vec::new();
        state_stack.push(Box::new(menu));

        Game {debug, renderer, state_stack}
    }

    pub fn run(&mut self, mut event_pump: sdl2::EventPump) {
        'running: loop {
            let curr_state = self.state_stack.last_mut().unwrap();
            let mut prev_time = time::Instant::now();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => curr_state.input_handler(event),
                }
            }

            curr_state.run();
            self.debug.run(curr_state.get_mut_world());
            self.renderer.run(curr_state.get_mut_world());
            curr_state.get_mut_world().maintain();
            let mut curr_time = time::Instant::now();
            if time::Duration::new(0, 1_000_000_000u32 / FRAMERATE) > curr_time.duration_since(prev_time) {
                ::std::thread::sleep(time::Duration::new(0, 1_000_000_000u32 / FRAMERATE) - curr_time.duration_since(prev_time));
            }
        }
    }
}

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Cursor;

impl Component for Cursor {
    type Storage = NullStorage<Self>;
}
