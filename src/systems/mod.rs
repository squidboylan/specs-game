use crate::game::input::*;
use crate::game::*;
use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Rect>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut rect, vel): Self::SystemData) {
        for (rect, vel) in (&mut rect, &vel).join() {
            rect.x += vel.x;
            rect.y += vel.y;
        }
    }
}

#[derive(SystemData)]
pub struct InputSystemData<'a> {
    rect: WriteStorage<'a, Rect>,
    rect_color: WriteStorage<'a, RectColor>,
    hover: WriteStorage<'a, Hover>,
    click: WriteStorage<'a, OnClick>,
    cursor: ReadStorage<'a, Cursor>,
    input: Write<'a, Input>,
    transition: Write<'a, Option<StateTransition>>,
    vel: WriteStorage<'a, Vel>,
    player: ReadStorage<'a, Player>,
}

pub struct InputHandler;

impl<'a> System<'a> for InputHandler {
    type SystemData = InputSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for (r, c, hover) in (&data.rect, &mut data.rect_color, &mut data.hover).join() {
            if data.input.mouse.x >= r.x
                && data.input.mouse.x <= r.x + r.w
                && data.input.mouse.y >= r.y
                && data.input.mouse.y <= r.y + r.h
            {
                *data.transition = hover.on_hover(c);
                match &*data.transition {
                    Some(_x) => return,
                    None => (),
                }
            } else {
                *data.transition = hover.off_hover(c);
                match &*data.transition {
                    Some(_x) => return,
                    None => (),
                }
            }
        }
        if data.input.mouse.left_tap {
            data.input.mouse.left_tap = false;
            for (r, _c, on_click) in (&data.rect, &mut data.rect_color, &mut data.click).join() {
                if data.input.mouse.x >= r.x
                    && data.input.mouse.x <= r.x + r.w
                    && data.input.mouse.y >= r.y
                    && data.input.mouse.y <= r.y + r.h
                {
                    *data.transition = (on_click.f)();
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

        let velocity = 2.0;

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
}

pub struct Creator {
    y: f32,
}

impl Creator {
    pub fn new(y: f32) -> Self {
        Creator { y }
    }
}

impl<'a> System<'a> for Creator {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, lazy): Self::SystemData) {
        let i = entities.create();
        let r = Rect::new(0.0, self.y, 10.0, 10.0);
        let c = RectColor::new(0, self.y as u8, 0, 255);
        lazy.insert(i, r);
        lazy.insert(i, c);
        lazy.insert(i, Vel { x: 2.0, y: 0.0 });
        self.y += 1.0;
    }
}
