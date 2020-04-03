use specs::prelude::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

struct Name(String);

impl Component for Name {
    type Storage = VecStorage<Self>;
}

struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Clone)]
struct Rect(sdl2::rect::Rect);

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rect(sdl2::rect::Rect::new(x, y, width, height))
    }
}

impl Component for Rect {
    type Storage = DenseVecStorage<Self>;
}

struct Renderer {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl<'a> System<'a> for Renderer {
    type SystemData = ReadStorage<'a, Rect>;

    fn run(&mut self, rect: Self::SystemData) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        for i in rect.as_slice() {
            self.canvas.fill_rect(i.0).unwrap();
        }
        self.canvas.present();
    }
}


struct Physics;

impl<'a> System<'a> for Physics {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteStorage<'a, Rect>, ReadStorage<'a, Vel>, ReadStorage<'a, Name>);

    fn run(&mut self, (mut rect, vel, _name): Self::SystemData) {
        for (rect, vel) in (&mut rect, &vel).join() {
            rect.0.set_x(rect.0.x() + vel.0 as i32);
        }
    }
}

struct Creator;

impl<'a> System<'a> for Creator {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (Entities<'a>, WriteStorage<'a, Rect>, WriteStorage<'a, Vel>);

    fn run(&mut self, (mut entities, mut rect, mut vel): Self::SystemData) {
        let i = entities.create();
        let r = Rect::new(0, 200, 10, 10);
        rect.insert(i, r).unwrap();
        vel.insert(i, Vel(2.0)).unwrap();
    }
}

fn main() -> Result<(), String> {
    // The `World` is our
    // container for components
    // and other resources.

    let mut world = World::new();
    world.register::<Rect>();
    world.register::<Vel>();
    world.register::<Name>();

    // An entity may or may not contain some component.
    let rect = Rect::new(0, 1, 5, 5);

    world.create_entity().with(Name("A".to_string())).with(Vel(2.0)).with(rect.clone()).build();
    world.create_entity().with(Name("B".to_string())).with(Vel(4.0)).with(rect.clone()).build();
    world.create_entity().with(Name("C".to_string())).with(Vel(1.5)).with(rect.clone()).build();

    // This builds a dispatcher.
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let mut dispatcher = DispatcherBuilder::new()
        .with(Physics, "physics", &[])
        .with(Creator, "creator", &[])
        .with_thread_local(Renderer{canvas}).build();

    // This dispatches all the systems in parallel (but blocking).


    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        dispatcher.dispatch(&mut world);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}
