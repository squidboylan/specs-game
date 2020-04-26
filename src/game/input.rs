use specs::prelude::*;
use crate::renderer::Rect;

#[derive(Default)]
pub struct Keyboard {
    pub W: bool,
    pub A: bool,
    pub S: bool,
    pub D: bool,
}

#[derive(Default)]
pub struct Mouse {
    pub x: i32,
    pub y: i32,
    pub left_tap: bool,
}

#[derive(Default)]
pub struct Input {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        Input {
            keyboard: Keyboard {
                W: false,
                A: false,
                S: false,
                D: false,
            },
            mouse: Mouse {
                x: 0,
                y: 0,
                left_tap: false,
            }
        }
    }

}

