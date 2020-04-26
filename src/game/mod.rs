use specs::prelude::*;
use crate::renderer;
use crate::debug;
use sfml::window::Event;
use sfml::window::Key;
use sfml::window::mouse::Button;
use std::time;

mod level;
mod menu;
mod input;

const FRAMERATE: u32 = 60;

pub trait GameState {
    fn input_handler(&mut self, event: Event) {
        match event {
            Event::KeyReleased { code: Key::W, ..} => self.get_mut_world().fetch_mut::<input::Input>().keyboard.W = false,
            Event::KeyReleased { code: Key::A, ..} => self.get_mut_world().fetch_mut::<input::Input>().keyboard.A = false,
            Event::KeyReleased { code: Key::S, ..} => self.get_mut_world().fetch_mut::<input::Input>().keyboard.S = false,
            Event::KeyReleased { code: Key::D, ..} => self.get_mut_world().fetch_mut::<input::Input>().keyboard.D = false,
            Event::KeyPressed { code: Key::W, ..} => self.get_mut_world().fetch_mut::<input::Input>().keyboard.W = true,
            Event::KeyPressed { code: Key::A, ..} => self.get_mut_world().fetch_mut::<input::Input>().keyboard.A = true,
            Event::KeyPressed { code: Key::S, ..} => self.get_mut_world().fetch_mut::<input::Input>().keyboard.S = true,
            Event::KeyPressed { code: Key::D, ..} => self.get_mut_world().fetch_mut::<input::Input>().keyboard.D = true,
            Event::KeyPressed { code: Key::Escape, .. } => *self.get_mut_world().fetch_mut::<Option<StateTransition>>() = Some(StateTransition::Pop),
            Event::MouseMoved { x, y, ..} => { let mut input = self.get_mut_world().fetch_mut::<input::Input>(); input.mouse.x = x; input.mouse.y = y },
            Event::MouseButtonPressed { button: Button::Left, ..} => self.get_mut_world().fetch_mut::<input::Input>().mouse.left_tap = true,
            _ => println!("{:?}", event),
        };
    }

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
    renderer: renderer::Renderer,
    state_stack: Vec<Box<dyn GameState>>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Self {
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
            .with(menu::OnClick{f: Box::new(|| {
                use crate::renderer::Rect;
                use crate::renderer::RectColor;
                use level::physics::*;
                use input::Input;

                let mut world = World::new();

                world.insert(Input::new());
                world.insert::<Option<StateTransition>>(None);

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
            .with(menu::OnHover{f: Box::new(|c| {
                c.0.r = 255;
                c.0.g = 255;
                c.0.b = 255;
                None
            })})
            .build();
        let mut debug = debug::Debug::new(&mut menu.world);
        let mut renderer = renderer::Renderer::new();
        let mut state_stack: Vec<Box<dyn GameState>> = Vec::new();
        state_stack.push(Box::new(menu));

        Game {debug, renderer, state_stack}
    }

    pub fn run(&mut self) {
        'running: loop {
            let mut prev_time = time::Instant::now();
            let mut transition = {
                let curr_state = self.state_stack.last_mut().unwrap();
                while let Some(event) = self.renderer.window.poll_event() {
                    match event {
                        Event::Closed {..} => {
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
