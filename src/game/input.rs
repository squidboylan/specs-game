

#[derive(Default)]
pub struct Keyboard {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

#[derive(Default)]
pub struct Mouse {
    pub x: f32,
    pub y: f32,
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
                w: false,
                a: false,
                s: false,
                d: false,
            },
            mouse: Mouse {
                x: 0.0,
                y: 0.0,
                left_tap: false,
            }
        }
    }

}

