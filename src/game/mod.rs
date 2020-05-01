use specs::prelude::*;
use crate::renderer;
use crate::debug;
use crate::components::*;
use ggez::{self, *};
use ggez::event::KeyCode;
use ggez::event::MouseButton;


mod level;
mod menu;
mod input;

pub trait GameState {
    fn get_mut_world(&mut self) -> &mut World;

    fn run(&mut self) -> Option<StateTransition>;
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
    pub fn new(ctx: &mut Context) -> Self {
        let mut menu = menu::Menu::new();
        let cursor_rect = Rect::new(0.0, 0.0, 5.0, 5.0);
        let rect = Rect::new(25.0, 25.0, 25.0, 25.0);
        let color = RectColor::new(255, 0, 0, 255);
        let cursor_color = RectColor::new(255, 255, 255, 255);

        menu.world.create_entity()
            .with(Cursor)
            .with(cursor_rect)
            .with(cursor_color)
            .build();
        menu.world.create_entity()
            .with(rect)
            .with(color)
            .with(Text("Hi".to_string()))
            .with(OnClick{f: Box::new(|| {
                use input::Input;

                let mut world = World::new();

                world.insert(Input::new());
                world.insert::<Option<StateTransition>>(None);

                world.register::<Rect>();
                world.register::<RectColor>();
                world.register::<Vel>();
                world.register::<Player>();
                world.register::<Cursor>();

                let rect = Rect::new(0.0, 1.0, 5.0, 5.0);
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
                    .with(cursor_color)
                    .build();
                world.create_entity()
                    .with(Vel{x: 1.0, y: 0.0})
                    .with(rect.clone())
                    .with(color.clone())
                    .build();
                world.create_entity()
                    .with(Vel{x: 0.0, y: 2.0})
                    .with(rect)
                    .with(color)
                    .build();
                Some(StateTransition::Push(State::Level, world))
            })})
            .with(OnHover{f: Box::new(|c| {
                c.0.r = 255.0;
                c.0.g = 255.0;
                c.0.b = 255.0;
                None
            })})
            .build();
        let debug = debug::Debug::new(&mut menu.world);
        let renderer = renderer::Renderer::new(ctx);
        let mut state_stack: Vec<Box<dyn GameState>> = Vec::new();
        state_stack.push(Box::new(menu));

        Game {debug, renderer, state_stack}
    }
}

impl<'a, 'b> event::EventHandler for Game<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let transition = {
            if self.state_stack.is_empty() {
                ggez::event::quit(ctx);
                return Ok(());
            }
            let curr_state = self.state_stack.last_mut().unwrap();
            let t = curr_state.run();
            self.debug.run(curr_state.get_mut_world());
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
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.state_stack.is_empty() {
            ggez::event::quit(ctx);
            return Ok(());
        }
        let curr_state = self.state_stack.last_mut().unwrap();
        self.renderer.run(ctx, curr_state.get_mut_world());
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        _repeat: bool,
        ) {
        if self.state_stack.is_empty() {
            ggez::event::quit(ctx);
        }
        let curr_state = self.state_stack.last_mut().unwrap();
        match keycode {
            KeyCode::W => curr_state.get_mut_world().fetch_mut::<input::Input>().keyboard.w = true,
            KeyCode::A => curr_state.get_mut_world().fetch_mut::<input::Input>().keyboard.a = true,
            KeyCode::S => curr_state.get_mut_world().fetch_mut::<input::Input>().keyboard.s = true,
            KeyCode::D => curr_state.get_mut_world().fetch_mut::<input::Input>().keyboard.d = true,
            KeyCode::Escape => *curr_state.get_mut_world().fetch_mut::<Option<StateTransition>>() = Some(StateTransition::Pop),
            _ => println!("Pressed: {:?}", keycode),
        };
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        ) {
        if self.state_stack.is_empty() {
            ggez::event::quit(ctx);
            return;
        }
        let curr_state = self.state_stack.last_mut().unwrap();
        match keycode {
            KeyCode::W => curr_state.get_mut_world().fetch_mut::<input::Input>().keyboard.w = false,
            KeyCode::A => curr_state.get_mut_world().fetch_mut::<input::Input>().keyboard.a = false,
            KeyCode::S => curr_state.get_mut_world().fetch_mut::<input::Input>().keyboard.s = false,
            KeyCode::D => curr_state.get_mut_world().fetch_mut::<input::Input>().keyboard.d = false,
            _ => println!("Released: {:?}", keycode),
        };
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32
        ) {
        if self.state_stack.is_empty() {
            ggez::event::quit(ctx);
            return;
        }
        let curr_state = self.state_stack.last_mut().unwrap();
        let mut input = curr_state.get_mut_world().fetch_mut::<input::Input>();
        input.mouse.x = x;
        input.mouse.y = y;
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32
        ) {
        if self.state_stack.is_empty() {
            ggez::event::quit(ctx);
            return;
        }
        let curr_state = self.state_stack.last_mut().unwrap();
        curr_state.get_mut_world().fetch_mut::<input::Input>();
        match button {
            MouseButton::Left => curr_state.get_mut_world().fetch_mut::<input::Input>().mouse.left_tap = true,
            _ => println!("Mouse Button Pressed: {:?}", button),
        };
    }
}
