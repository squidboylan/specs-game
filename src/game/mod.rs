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

    fn run(&mut self) -> Option<StateTransition> ;
}

pub enum StateTransition {
    Push(State, World),
    Pop,
}

pub enum State {
    Level,
    Menu
}

pub struct Game<'a, 'b> {
    debug: debug::Debug<'a, 'b>,
    renderer: renderer::Renderer<'a, 'b>,
    state_stack: Vec<Box<dyn GameState>>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(sdl_context: &sdl2::Sdl, ttf_context: &'b sdl2::ttf::Sdl2TtfContext) -> Self {
        let mut menu = menu::Menu::new();
        let cursor_rect = renderer::Rect::new(0, 0, 5, 5);
        let rect = renderer::Rect::new(25, 25, 25, 25);
        let color = renderer::RectColor::new(255, 0, 0, 255);
        let cursor_color = renderer::RectColor::new(255, 255, 255, 255);

        menu.world.create_entity()
            .with(Cursor)
            .with(cursor_rect.clone())
            .with(cursor_color.clone())
            .build();
        menu.world.create_entity()
            .with(rect.clone())
            .with(color.clone())
            .with(menu::OnHover{f: Box::new(|c| {
                use crate::renderer::Rect;
                use crate::renderer::RectColor;
                use level::physics::*;
                use input::Input;

                let mut world = World::new();

                world.insert(Input::new());

                world.register::<Rect>();
                world.register::<RectColor>();
                world.register::<Vel>();
                world.register::<Player>();
                world.register::<Cursor>();

                let rect = Rect::new(0, 1, 5, 5);
                let color = RectColor::new(255, 0, 0, 255);
                let cursor_color = RectColor::new(255, 255, 255, 255);

                world.create_entity()
                    .with(Player)
                    .with(Vel{x: 0.0, y: 0.0})
                    .with(rect.clone())
                    .with(color.clone())
                    .build();
                world.create_entity()
                    .with(Cursor)
                    .with(Vel{x: 0.0, y: 0.0})
                    .with(rect.clone())
                    .with(cursor_color.clone())
                    .build();
                world.create_entity()
                    .with(Vel{x: 1.0, y: 0.0})
                    .with(rect.clone())
                    .with(color.clone())
                    .build();
                world.create_entity()
                    .with(Vel{x: 0.0, y: 2.0})
                    .with(rect.clone())
                    .with(color.clone())
                    .build();
                Some(StateTransition::Push(State::Level, world))
            })})
            .build();
        let mut debug = debug::Debug::new(&mut menu.world);
        let mut renderer = renderer::Renderer::new(sdl_context, ttf_context);
        let mut state_stack: Vec<Box<dyn GameState>> = Vec::new();
        state_stack.push(Box::new(menu));

        Game {debug, renderer, state_stack}
    }

    pub fn run(&mut self, mut event_pump: sdl2::EventPump) {
        'running: loop {
            let mut prev_time = time::Instant::now();
            let mut transition = {
                let curr_state = self.state_stack.last_mut().unwrap();
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running
                        },
                        _ => curr_state.input_handler(event),
                    }
                }

                let t = curr_state.run();
                self.debug.run(curr_state.get_mut_world());
                self.renderer.run(curr_state.get_mut_world());
                curr_state.get_mut_world().maintain();
                t
            };

            match transition {
                Some(StateTransition::Push(State::Level, mut world)) => {
                    self.debug = debug::Debug::new(&mut world);
                    self.state_stack.push(Box::new(level::Level::from_world(world)));
                },
                Some(StateTransition::Push(State::Menu, mut world)) => {
                    self.debug = debug::Debug::new(&mut world);
                    self.state_stack.push(Box::new(menu::Menu::from_world(world)));
                },
                Some(StateTransition::Pop) => { self.state_stack.pop(); },
                None => (),
            };
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
