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
pub struct ParticleSystemData<'a> {
    rect: WriteStorage<'a, Rect>,
    cursor: ReadStorage<'a, Cursor>,
    player: ReadStorage<'a, Player>,
    rotation: WriteStorage<'a, Rotation>,
    particle_engine: Write<'a, particles::ParticleEngine>,
    input: Read<'a, Input>,
}

pub struct ParticleSystem;

impl<'a> System<'a> for ParticleSystem {
    type SystemData = ParticleSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.input.mouse.left_down {
            for (_, player_rect, mut rotation) in (&data.player, &data.rect, &mut data.rotation).join() {
                let (x, y) = player_rect.get_center();
                let vel = 5.0;
                let p = particles::Particle {
                    location: (x, y, 0.0, 0.0),
                    color: (1.0, 1.0, 1.0, 1.0),
                    dimensions: (5.0, 5.0),
                    accel: (0.0, 0.0),
                    velocity: (vel * rotation.cos(), vel * rotation.sin()),
                    life: 1200,
                    pad: 0,
                };
                data.particle_engine.create_particle(p);
            }
        }
    }
}
