use crate::game::input::*;
use crate::game::*;
use specs::prelude::*;
use std::f32::consts::PI;

use crate::components::*;

#[derive(SystemData)]
pub struct PhysicsSystemData<'a> {
    rect: WriteStorage<'a, Rect>,
    cursor: ReadStorage<'a, Cursor>,
    vel: ReadStorage<'a, Vel>,
    player: ReadStorage<'a, Player>,
    rotation: WriteStorage<'a, Rotation>,
}

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = PhysicsSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (rect, vel) in (&mut data.rect, &data.vel).join() {
            rect.x += vel.x;
            rect.y += vel.y;
        }
        for (_, player_rect, mut rotation) in (&data.player, &data.rect, &mut data.rotation).join() {
            for (_, cursor_rect) in (&data.cursor, &data.rect).join() {
                let player_center = player_rect.get_center();
                let cursor_center = cursor_rect.get_center();
                let new_vec = (cursor_center.0 - player_center.0, cursor_center.1 - player_center.1);
                rotation.0 = new_vec.1.atan2(new_vec.0);
            }
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
}
