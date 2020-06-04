use std::vec::Drain;
use std::ops::RangeBounds;

#[derive(Default, Clone)]
pub struct Particle {
    pub location: (f32, f32, f32, f32),
    pub color: (f32, f32, f32, f32),
    pub dimensions: (f32, f32),
    pub accel: (f32, f32),
    pub velocity: (f32, f32),
    pub life: u32,
    pub pad: u32,
}

#[derive(Default, Clone)]
pub struct ParticleEngine {
    pub particles: Vec<Particle>,
}

impl ParticleEngine {
    pub fn new() -> Self {
        ParticleEngine {
            // This doesnt limit us to 1024 particles per frame, but if we create more than that in
            // a single frame we'll have to grow the vec, I think that's fine.
            particles: Vec::with_capacity(1024)
        }
    }

    pub fn create_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }
}
