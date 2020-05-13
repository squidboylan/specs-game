use crate::components::*;
use crate::debug;
use crate::renderer;
use crate::systems::*;
use ggez::event::KeyCode;
use ggez::event::MouseButton;
use ggez::{self, *};
use specs::prelude::*;
use std::mem;

pub mod input;
pub mod map;

struct GameState {
    pub world: World,
}

impl GameState {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    pub fn initialized_world() -> World {
        let mut menu_world = World::new();
        menu_world.insert(input::Input::new());
        menu_world.insert::<Option<StateTransition>>(None);

        menu_world.register::<Rect>();
        menu_world.register::<RectColor>();
        menu_world.register::<Rotation>();
        menu_world.register::<Vel>();
        menu_world.register::<Text>();
        menu_world.register::<Hover>();
        menu_world.register::<OnClick>();
        menu_world.register::<Player>();
        menu_world.register::<Cursor>();
        menu_world
    }
}

pub enum StateTransition {
    Push(World),
    Pop,
}

pub struct Game<'a, 'b> {
    debug: debug::Debug<'a, 'b>,
    renderer: renderer::Renderer,
    state_stack: Vec<Box<GameState>>,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> Self {
        let mut menu_world = GameState::initialized_world();
        let cursor_rect = Rect::new(0.0, 0.0, 5.0, 5.0);
        let rect = Rect::new(
            renderer::SCREEN_WIDTH / 2.0 - 200.0 / 2.0,
            200.0,
            100.0,
            50.0,
        );
        let color = RectColor::new(255, 0, 0, 255);
        let cursor_color = RectColor::new(255, 255, 255, 255);
        let level_data = tiled::parse(ggez::filesystem::open(ctx, "/map1.tmx").unwrap()).unwrap();
        let map = map::Map::new(ctx, &level_data);

        menu_world
            .create_entity()
            .with(Cursor)
            .with(cursor_rect)
            .with(cursor_color)
            .build();
        menu_world
            .create_entity()
            .with(rect)
            .with(color)
            .with(Text {
                text: "Level 1".to_string(),
                scale: graphics::Scale::uniform(25.0),
            })
            .with(OnClick {
                f: Box::new(move || {
                    let mut world = GameState::initialized_world();
                    world.insert(map.clone());

                    let player_rect = Rect::new(0.0, 0.0, 25.0, 25.0);
                    let rect = Rect::new(0.0, 1.0, 5.0, 5.0);
                    let color = RectColor::new(255, 0, 0, 255);
                    let cursor_color = RectColor::new(255, 255, 255, 255);

                    world
                        .create_entity()
                        .with(Player)
                        .with(Rotation(0.0))
                        .with(Vel { x: 0.0, y: 0.0 })
                        .with(player_rect)
                        .with(color.clone())
                        .build();
                    world
                        .create_entity()
                        .with(Cursor)
                        .with(Vel { x: 0.0, y: 0.0 })
                        .with(rect.clone())
                        .with(cursor_color)
                        .build();
                    world
                        .create_entity()
                        .with(Vel { x: 1.0, y: 0.0 })
                        .with(rect.clone())
                        .with(color.clone())
                        .build();
                    world
                        .create_entity()
                        .with(Vel { x: 0.0, y: 2.0 })
                        .with(rect)
                        .with(color)
                        .build();
                    Some(StateTransition::Push(world))
                }),
            })
            .with(Hover::new(
                Box::new(|c| {
                    c.0.r = 255.0;
                    c.0.g = 255.0;
                    c.0.b = 255.0;
                    None
                }),
                Box::new(|c| {
                    c.0.r = 255.0;
                    c.0.g = 0.0;
                    c.0.b = 0.0;
                    None
                }),
            ))
            .build();
        let dispatcher = DispatcherBuilder::new()
            .with(InputHandler, "input", &[])
            //.with(Creator::new(0.0), "Creator", &[])
            .with(Physics, "physics", &["input"])
            .build();
        let debug = debug::Debug::new(&mut menu_world);
        let menu = GameState::new(menu_world);
        let renderer = renderer::Renderer::new(ctx);
        let mut state_stack: Vec<Box<GameState>> = Vec::new();
        state_stack.push(Box::new(menu));

        Game {
            debug,
            renderer,
            state_stack,
            dispatcher,
        }
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
            let t = {
                if curr_state.world.fetch_mut::<Option<StateTransition>>().is_none() {
                    self.dispatcher.dispatch(&curr_state.world);
                }
                mem::replace(
                    &mut *curr_state.world.fetch_mut::<Option<StateTransition>>(),
                    None,
                )
            };
            self.debug.run(&mut curr_state.world);
            curr_state.world.maintain();
            t
        };

        match transition {
            Some(StateTransition::Push(mut world)) => {
                self.debug = debug::Debug::new(&mut world);
                self.state_stack
                    .push(Box::new(GameState::new(world)));
            }
            Some(StateTransition::Pop) => {
                self.state_stack.pop();
            }
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
        self.renderer.run(ctx, &mut curr_state.world);
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
            KeyCode::W => curr_state.world.fetch_mut::<input::Input>().keyboard.w = true,
            KeyCode::A => curr_state.world.fetch_mut::<input::Input>().keyboard.a = true,
            KeyCode::S => curr_state.world.fetch_mut::<input::Input>().keyboard.s = true,
            KeyCode::D => curr_state.world.fetch_mut::<input::Input>().keyboard.d = true,
            KeyCode::Escape => {
                *curr_state.world.fetch_mut::<Option<StateTransition>>() =
                    Some(StateTransition::Pop)
            }
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
            KeyCode::W => curr_state.world.fetch_mut::<input::Input>().keyboard.w = false,
            KeyCode::A => curr_state.world.fetch_mut::<input::Input>().keyboard.a = false,
            KeyCode::S => curr_state.world.fetch_mut::<input::Input>().keyboard.s = false,
            KeyCode::D => curr_state.world.fetch_mut::<input::Input>().keyboard.d = false,
            _ => println!("Released: {:?}", keycode),
        };
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if self.state_stack.is_empty() {
            ggez::event::quit(ctx);
            return;
        }
        let curr_state = self.state_stack.last_mut().unwrap();
        let mut input = curr_state.world.fetch_mut::<input::Input>();
        input.mouse.x = x;
        input.mouse.y = y;
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if self.state_stack.is_empty() {
            ggez::event::quit(ctx);
            return;
        }
        let curr_state = self.state_stack.last_mut().unwrap();
        match button {
            MouseButton::Left => curr_state.world.fetch_mut::<input::Input>().mouse.left_tap = true,
            _ => println!("Mouse Button Pressed: {:?}", button),
        };
    }
}
