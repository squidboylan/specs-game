use crate::components::*;
use crate::debug;
use crate::renderer;
use crate::systems::*;
use crate::game::input::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::event::VirtualKeyCode;
use glutin::WindowedContext;
use specs::prelude::*;
use std::mem;

pub mod input;
pub mod particles;
pub mod map;

#[derive(SystemData)]
struct InputSystemData<'a> {
    entities: Entities<'a>,
    rect: WriteStorage<'a, Rect>,
    hover: WriteStorage<'a, Hover>,
    click: WriteStorage<'a, OnClick>,
    cursor: ReadStorage<'a, Cursor>,
    input: Write<'a, Input>,
    transition: Write<'a, Option<StateTransition>>,
    vel: WriteStorage<'a, Vel>,
    player: ReadStorage<'a, Player>,
    particle_engine: Write<'a, particles::ParticleEngine>,
}

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
    debug: debug::Debug,
    renderer: renderer::Renderer,
    state_stack: Vec<Box<GameState>>,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Self {
        let mut menu_world = GameState::initialized_world();
        let particle_engine = particles::ParticleEngine::new();
        let cursor_rect = Rect::new(0.0, 0.0, 5.0, 5.0);
        let rect = Rect::new(
            renderer::SCREEN_WIDTH / 2.0 - 200.0 / 2.0,
            200.0,
            100.0,
            50.0,
        );
        let color = RectColor::new(1.0, 0.0, 0.0, 1.0);
        let cursor_color = RectColor::new(1.0, 1.0, 1.0, 1.0);
        let level_data = tiled::parse_file(std::path::Path::new("./resources/map1.tmx")).unwrap();
        let map = map::Map::from_tiled(&level_data).unwrap();

        menu_world.insert(particle_engine);

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
                text: "Levelp".to_string(),
                scale: 1.0,
                location: (rect.x + 5.0, rect.y + 35.0),
            })
            .with(OnClick {
                f: Box::new(move |_, _| {
                    let mut world = GameState::initialized_world();
                    let particle_engine = particles::ParticleEngine::new();
                    world.insert(map.clone());
                    world.insert(particle_engine);

                    let player_rect = Rect::new(0.0, 0.0, 25.0, 25.0);
                    let rect = Rect::new(0.0, 1.0, 5.0, 5.0);
                    let color = RectColor::new(1.0, 0.0, 0.0, 1.0);
                    let cursor_color = RectColor::new(1.0, 1.0, 1.0, 1.0);

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
                Box::new(|w, e| {
                    let mut cs = w.write_component::<RectColor>();
                    let c = cs.get_mut(e).unwrap();

                    c.r = 1.0;
                    c.g = 1.0;
                    c.b = 1.0;
                    None
                }),
                Box::new(|w, e| {
                    let mut cs = w.write_component::<RectColor>();
                    let c = cs.get_mut(e).unwrap();

                    c.r = 1.0;
                    c.g = 0.0;
                    c.b = 0.0;
                    None
                }),
            ))
            .build();
        let dispatcher = DispatcherBuilder::new()
            .with(Physics, "physics", &[])
            .with(ParticleSystem::new(), "particles", &["physics"])
            .build();
        let debug = debug::Debug::new(&mut menu_world);
        let menu = GameState::new(menu_world);
        let renderer = renderer::Renderer::new();
        let mut state_stack: Vec<Box<GameState>> = Vec::new();
        state_stack.push(Box::new(menu));

        Game {
            debug,
            renderer,
            state_stack,
            dispatcher,
        }
    }

    pub fn update_input(&mut self) {
        let curr_state = self.state_stack.last_mut().unwrap();
        let mut data: InputSystemData = curr_state.world.system_data();
        for (e, r, hover) in (&data.entities, &data.rect, &mut data.hover).join() {
            if data.input.mouse.x >= r.x
                && data.input.mouse.x <= r.x + r.w
                && data.input.mouse.y >= r.y
                && data.input.mouse.y <= r.y + r.h
            {
                let tmp_transition = hover.on_hover(&curr_state.world, e);
                match &tmp_transition {
                    Some(_x) => { *data.transition = tmp_transition; return },
                    None => (),
                }
            } else {
                let tmp_transition = hover.off_hover(&curr_state.world, e);
                match &tmp_transition {
                    Some(_x) => { *data.transition = tmp_transition; return },
                    None => (),
                }
            }
        }
        if data.input.mouse.left_tap {
            data.input.mouse.left_tap = false;
            for (e, r, on_click) in (&data.entities, &data.rect, &mut data.click).join() {
                if data.input.mouse.x >= r.x
                    && data.input.mouse.x <= r.x + r.w
                    && data.input.mouse.y >= r.y
                    && data.input.mouse.y <= r.y + r.h
                {
                    *data.transition = (on_click.f)(&curr_state.world, e);
                    match &*data.transition {
                        Some(_x) => return,
                        None => (),
                    }
                }
            }
        }
        for (r, _) in (&mut data.rect, &data.cursor).join() {
            r.x = data.input.mouse.x - r.w / 2.0;
            r.y = data.input.mouse.y - r.h / 2.0;
        }

        let velocity = 5.0;

        for (v, _) in (&mut data.vel, &data.player).join() {
            if data.input.keyboard.w {
                v.y = -1.0 * velocity;
            }
            if data.input.keyboard.a {
                v.x = -1.0 * velocity;
            }
            if data.input.keyboard.s {
                v.y = velocity;
            }
            if data.input.keyboard.d {
                v.x = velocity;
            }

            if !data.input.keyboard.w && !data.input.keyboard.s {
                v.y = 0.0;
            }
            if !data.input.keyboard.a && !data.input.keyboard.d {
                v.x = 0.0;
            }
        }

        for (r, _) in (&mut data.rect, &data.cursor).join() {
            r.x = data.input.mouse.x - r.w / 2.0;
            r.y = data.input.mouse.y - r.h / 2.0;
        }
    }

    pub fn update(&mut self) -> Option<glutin::event_loop::ControlFlow> {
        if self.state_stack.is_empty() {
            return Some(glutin::event_loop::ControlFlow::Exit);
        }

        self.update_input();
        let transition = {
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
        None
    }

    pub fn draw(&mut self, ctx: &mut WindowedContext<glutin::PossiblyCurrent>) -> Option<glutin::event_loop::ControlFlow> {
        if self.state_stack.is_empty() {
            return Some(glutin::event_loop::ControlFlow::Exit);
        }
        let curr_state = self.state_stack.last_mut().unwrap();
        self.renderer.run(ctx, &mut curr_state.world);
        None
    }

    pub fn key_event(
        &mut self,
        key_input: glutin::event::KeyboardInput,
    ) {
        let curr_state = self.state_stack.last_mut().unwrap();
        let key_state = key_input.state;
        if key_state == glutin::event::ElementState::Pressed {
            match key_input.virtual_keycode {
                Some(VirtualKeyCode::W) => curr_state.world.fetch_mut::<input::Input>().keyboard.w = true,
                Some(VirtualKeyCode::A) => curr_state.world.fetch_mut::<input::Input>().keyboard.a = true,
                Some(VirtualKeyCode::S) => curr_state.world.fetch_mut::<input::Input>().keyboard.s = true,
                Some(VirtualKeyCode::D) => curr_state.world.fetch_mut::<input::Input>().keyboard.d = true,
                Some(VirtualKeyCode::Escape) => {
                    *curr_state.world.fetch_mut::<Option<StateTransition>>() =
                        Some(StateTransition::Pop)
                }
                _ => println!("Pressed: {:?}", key_input.virtual_keycode),
            };
        } else {
            match key_input.virtual_keycode {
                Some(VirtualKeyCode::W) => curr_state.world.fetch_mut::<input::Input>().keyboard.w = false,
                Some(VirtualKeyCode::A) => curr_state.world.fetch_mut::<input::Input>().keyboard.a = false,
                Some(VirtualKeyCode::S) => curr_state.world.fetch_mut::<input::Input>().keyboard.s = false,
                Some(VirtualKeyCode::D) => curr_state.world.fetch_mut::<input::Input>().keyboard.d = false,
                _ => println!("Released: {:?}", key_input.virtual_keycode),
            };
        }
    }

    pub fn mouse_movement (
        &mut self,
        pos: glutin::dpi::PhysicalPosition<f64>,
    ) {
        let curr_state = self.state_stack.last_mut().unwrap();
        curr_state.world.fetch_mut::<input::Input>().mouse.x = pos.x as f32;
        curr_state.world.fetch_mut::<input::Input>().mouse.y = pos.y as f32;

        println!("{:?}", pos);
    }

    pub fn mouse_button_down_event(
        &mut self,
        button: glutin::event::MouseButton,
        state: glutin::event::ElementState,
    ) {
        let curr_state = self.state_stack.last_mut().unwrap();
        if state == glutin::event::ElementState::Pressed {
            match button {
                glutin::event::MouseButton::Left => { curr_state.world.fetch_mut::<input::Input>().mouse.left_tap = true; curr_state.world.fetch_mut::<input::Input>().mouse.left_down = true },
                _ => println!("Mouse Button Pressed: {:?}", button),
            };
        }

        if state == glutin::event::ElementState::Released {
            match button {
                glutin::event::MouseButton::Left => { curr_state.world.fetch_mut::<input::Input>().mouse.left_down = false },
                _ => println!("Mouse Button Pressed: {:?}", button),
            };
        }
    }
}
