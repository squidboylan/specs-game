use specs::prelude::*;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::game::StateTransition;

#[derive(Default)]
pub struct FPS;

impl Component for FPS {
    type Storage = NullStorage<Self>;
}

pub struct Text {
    pub text: String,
    pub scale: f32,
    pub location: (f32, f32),
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
pub struct Rotation(pub f32);

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}

impl Deref for Rotation {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rotation {
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.0
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Component for Rect {
    type Storage = VecStorage<Self>;
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn get_center(&self) -> (f32, f32) {
        (self.w/2.0 + self.x, self.h/2.0 + self.y)
    }
}

#[derive(Clone)]
pub struct RectColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Component for RectColor {
    type Storage = VecStorage<Self>;
}

impl RectColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a
        }
    }
}

pub struct Hover {
    pub on_hover_fn: Box<dyn FnMut(&World, specs::Entity) -> Option<StateTransition> + Send + Sync>,
    pub off_hover_fn: Box<dyn FnMut(&World, specs::Entity) -> Option<StateTransition> + Send + Sync>,
    hovering: bool,
}

impl Hover {
    pub fn new(
        on_hover_fn: Box<dyn FnMut(&World, specs::Entity) -> Option<StateTransition> + Send + Sync>,
        off_hover_fn: Box<dyn FnMut(&World, specs::Entity) -> Option<StateTransition> + Send + Sync>,
    ) -> Self {
        Hover {
            on_hover_fn,
            off_hover_fn,
            hovering: false,
        }
    }

    pub fn on_hover(&mut self, w: &World, e: specs::Entity) -> Option<StateTransition> {
        if !self.hovering {
            self.hovering = true;
            (self.on_hover_fn)(w, e)
        } else {
            None
        }
    }

    pub fn off_hover(&mut self, w: &World, e: specs::Entity) -> Option<StateTransition> {
        if self.hovering {
            self.hovering = false;
            (self.off_hover_fn)(w, e)
        } else {
            None
        }
    }
}

impl Component for Hover {
    type Storage = VecStorage<Self>;
}

pub struct OnClick {
    pub f: Box<dyn FnMut(&World, specs::Entity) -> Option<StateTransition> + Send + Sync>,
}

impl Component for OnClick {
    type Storage = VecStorage<Self>;
}

pub struct Vel {
    pub x: f32,
    pub y: f32,
}

impl Component for Vel {
    type Storage = VecStorage<Self>;
}
