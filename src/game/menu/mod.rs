use specs::prelude::*;
use crate::renderer::Rect;
use crate::renderer::RectColor;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::game::input::*;
use crate::game::*;

pub struct Menu<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    pub world: World
}

impl<'a, 'b> GameState for Menu<'a, 'b> {
    fn input_handler(&mut self, event: Event) {
        let mut input = self.world.fetch_mut::<Input>();
        match event {
            Event::KeyUp { keycode: Some(Keycode::W), ..} => input.keyboard.W = false,
            Event::KeyUp { keycode: Some(Keycode::A), ..} => input.keyboard.A = false,
            Event::KeyUp { keycode: Some(Keycode::S), ..} => input.keyboard.S = false,
            Event::KeyUp { keycode: Some(Keycode::D), ..} => input.keyboard.D = false,
            Event::KeyDown { keycode: Some(Keycode::W), ..} => input.keyboard.W = true,
            Event::KeyDown { keycode: Some(Keycode::A), ..} => input.keyboard.A = true,
            Event::KeyDown { keycode: Some(Keycode::S), ..} => input.keyboard.S = true,
            Event::KeyDown { keycode: Some(Keycode::D), ..} => input.keyboard.D = true,
            Event::MouseMotion { x: x, y: y, ..} => { input.mouse.x = x; input.mouse.y = y },
            _ => println!("{:?}", event),
        }
    }

    fn get_mut_world(&mut self) -> &mut World {
        &mut self.world
    }

    fn run(&mut self) {
        self.dispatcher.dispatch(&self.world);
    }
}

impl<'a, 'b> Menu<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();

        world.insert(Input::new());

        world.register::<Rect>();
        world.register::<RectColor>();
        world.register::<OnHover>();
        world.register::<Cursor>();

        let cursor_rect = Rect::new(0, 0, 5, 5);
        let rect = Rect::new(25, 25, 25, 25);
        let color = RectColor::new(255, 0, 0, 255);
        let cursor_color = RectColor::new(255, 255, 255, 255);

        world.create_entity()
            .with(Cursor)
            .with(cursor_rect.clone())
            .with(cursor_color.clone())
            .build();
        world.create_entity()
            .with(rect.clone())
            .with(color.clone())
            .with(OnHover{f: Box::new(|c| {
                c.0.r = 255;
                c.0.g = 255;
                c.0.b = 255;
            })})
            .build();
        let dispatcher = DispatcherBuilder::new()
            .with(InputHandler, "input_handler", &[])
            .build();

        Menu { dispatcher, world }
    }
}

pub struct InputHandler;

impl<'a> System<'a> for InputHandler {
    type SystemData = (WriteStorage<'a, Rect>, WriteStorage<'a, RectColor>, WriteStorage<'a, OnHover>, ReadStorage<'a, Cursor>, Read<'a, Input>);

    fn run(&mut self, (mut rect, mut color, mut hover, cursor, input): Self::SystemData) {
        for (r, c, on_hover) in (&rect, &mut color, &mut hover).join() {
            if input.mouse.x >= r.0.x && input.mouse.x <= r.0.x + r.0.w &&
                input.mouse.y >= r.0.y && input.mouse.y <= r.0.y + r.0.h {
                (on_hover.f)(c)
            }
        }
        for (r, _) in (&mut rect, &cursor).join() {
            r.0.x = input.mouse.x - r.0.w/2;
            r.0.y = input.mouse.y - r.0.h/2;
        }
    }
}

pub struct OnHover{
    pub f: Box<dyn FnMut(&mut RectColor) -> () + Send + Sync>,
}

impl Component for OnHover {
    type Storage = VecStorage<Self>;
}

