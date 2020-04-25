use specs::prelude::*;
use crate::renderer::Rect;
use crate::renderer::RectColor;
use sdl2::event::Event;

pub mod level;

pub trait GameState {
    fn new() -> Self;

    fn input_handler(&mut self, event: Event);
}
