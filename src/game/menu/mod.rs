use specs::prelude::*;
use crate::renderer::Rect;
use crate::renderer::RectColor;
use crate::game::input::*;
use crate::game::*;
use crate::game::level;
use std::mem;

pub struct Menu<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
}

impl<'a, 'b> GameState for Menu<'a, 'b> {
    fn get_mut_world(&mut self) -> &mut World {
        &mut self.world
    }

    fn run(&mut self) -> Option<StateTransition> {
        self.dispatcher.dispatch(&self.world);
        mem::replace(&mut *self.world.fetch_mut::<Option<StateTransition>>(), None)
    }
}

impl<'a, 'b> Menu<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();

        world.insert(Input::new());
        world.insert::<Option<StateTransition>>(None);

        world.register::<Rect>();
        world.register::<RectColor>();
        world.register::<OnHover>();
        world.register::<OnClick>();
        world.register::<Cursor>();

        let dispatcher = DispatcherBuilder::new()
            .with(InputHandler, "input_handler", &[])
            .build();

        Menu { dispatcher, world}
    }

    pub fn from_world(world: World) -> Self {
        let dispatcher = DispatcherBuilder::new()
            .with(InputHandler, "input_handler", &[])
            .build();

        Menu { dispatcher, world}
    }
}

pub struct InputHandler;

impl<'a> System<'a> for InputHandler {
    type SystemData = (WriteStorage<'a, Rect>, WriteStorage<'a, RectColor>, WriteStorage<'a, OnHover>, WriteStorage<'a, OnClick>, ReadStorage<'a, Cursor>, Write<'a, Input>, Write<'a, Option<StateTransition>>);

    fn run(&mut self, (mut rect, mut color, mut hover, mut click, cursor, mut input, mut trans): Self::SystemData) {
        for (r, c, on_hover) in (&rect, &mut color, &mut hover).join() {
            if input.mouse.x >= r.0.x && input.mouse.x <= r.0.x + r.0.w &&
                input.mouse.y >= r.0.y && input.mouse.y <= r.0.y + r.0.h {
                *trans = (on_hover.f)(c);
                match &*trans {
                    Some(x) => return,
                    None => (),
                }
            }
        }
        if input.mouse.left_tap {
            input.mouse.left_tap = false;
            for (r, c, on_click) in (&rect, &mut color, &mut click).join() {
                if input.mouse.x >= r.0.x && input.mouse.x <= r.0.x + r.0.w &&
                    input.mouse.y >= r.0.y && input.mouse.y <= r.0.y + r.0.h {
                    *trans = (on_click.f)();
                    match &*trans {
                        Some(x) => return,
                        None => (),
                    }
                }
            }
        }
        for (r, _) in (&mut rect, &cursor).join() {
            r.0.x = input.mouse.x - r.0.w/2.0;
            r.0.y = input.mouse.y - r.0.h/2.0;
        }
    }
}

pub struct OnHover{
    pub f: Box<dyn FnMut(&mut RectColor) -> Option<StateTransition> + Send + Sync>,
}

impl Component for OnHover {
    type Storage = VecStorage<Self>;
}

pub struct OnClick{
    pub f: Box<dyn FnMut() -> Option<StateTransition> + Send + Sync>,
}

impl Component for OnClick {
    type Storage = VecStorage<Self>;
}
