use specs::prelude::*;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::game::StateTransition;

#[derive(Default)]
pub struct FPS;

impl Component for FPS {
    type Storage = NullStorage<Self>;
}

pub struct Text{
    pub text: String,
    pub scale: ggez::graphics::Scale,
}

impl Component for Text {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Cursor;

impl Component for Cursor {
    type Storage = NullStorage<Self>;
}


#[derive(Clone)]
pub struct Rect(pub ggez::graphics::Rect);

impl Component for Rect {
    type Storage = VecStorage<Self>;
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect(ggez::graphics::Rect::new(x, y, width, height))
    }
}

impl Deref for Rect {
    type Target = ggez::graphics::Rect;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rect {
    fn deref_mut(&mut self) -> &mut ggez::graphics::Rect {
        &mut self.0
    }
}

#[derive(Clone)]
pub struct RectColor(pub ggez::graphics::Color);

impl Component for RectColor {
    type Storage = VecStorage<Self>;
}

impl RectColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RectColor(ggez::graphics::Color::from_rgba(r, g, b, a))
    }
}

impl Deref for RectColor {
    type Target = ggez::graphics::Color;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RectColor {
    fn deref_mut(&mut self) -> &mut ggez::graphics::Color {
        &mut self.0
    }
}

pub struct Hover {
    pub on_hover_fn: Box<dyn FnMut(&mut RectColor) -> Option<StateTransition> + Send + Sync>,
    pub off_hover_fn: Box<dyn FnMut(&mut RectColor) -> Option<StateTransition> + Send + Sync>,
    hovering: bool,
}

impl  Hover {
    pub fn new(on_hover_fn: Box<dyn FnMut(&mut RectColor) -> Option<StateTransition> + Send + Sync>, off_hover_fn: Box<dyn FnMut(&mut RectColor) -> Option<StateTransition> + Send + Sync>) -> Self {
        Hover {
            on_hover_fn,
            off_hover_fn,
            hovering: false,
        }
    }

    pub fn on_hover(&mut self, c: &mut RectColor) -> Option<StateTransition> {
        if !self.hovering {
            self.hovering = true;
            (self.on_hover_fn)(c)
        } else {
            None
        }
    }

    pub fn off_hover(&mut self, c: &mut RectColor) -> Option<StateTransition> {
        if self.hovering {
            self.hovering = false;
            (self.off_hover_fn)(c)
        } else {
            None
        }
    }
}

impl Component for Hover {
    type Storage = VecStorage<Self>;
}

pub struct OnClick {
    pub f: Box<dyn FnMut() -> Option<StateTransition> + Send + Sync>,
}

impl Component for OnClick {
    type Storage = VecStorage<Self>;
}

pub struct Vel{
    pub x: f32,
    pub y: f32,
}

impl Component for Vel {
    type Storage = VecStorage<Self>;
}
