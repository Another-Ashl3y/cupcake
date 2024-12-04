use minifb;
use std::time::Instant;
use crate::engine::particle::Particle;

use super::BoxedParticle;

pub struct Frame {
    pub window:             minifb::Window,
    pub buffer:             Vec<u32>,

    pub particles:          ParticleHandler,
    pub target_fps:         usize,
}

impl Frame {
    pub fn new(name: &str, width: usize, height: usize, window_options: minifb::WindowOptions) -> Result<Frame, minifb::Error> {
        Ok(Self {
            window:         minifb::Window::new(name, width, height, window_options)?,
            buffer:         vec![0; width*height],
            particles:      ParticleHandler::new(),
            target_fps:     60,
        })
    }
    pub fn update(&mut self, delta: f64) {
        self.particles.update(delta);
    }

    pub fn run(&mut self) -> Result<(), minifb::Error> {
        let mut before = Instant::now();
        let mut delta = 0.0;
        while self.window.is_open() {


            let dimensions = self.window.get_size();
    
            {
                // Code
                self.update(delta);
            }

            self.window.update_with_buffer(&self.buffer, dimensions.0, dimensions.1)?;

            // Correct buffer size if window dimensions have been changed.
            let updated_dimensions = self.window.get_size();
            if dimensions != updated_dimensions {
                self.buffer = vec![0; updated_dimensions.0 * updated_dimensions.1];
            }

            delta = before.elapsed().as_secs_f64();
            before = Instant::now();
        }
        Ok(())
    }
}


pub struct ParticleHandler {
    pub active_particles:   Vec<BoxedParticle>,         // Vector array of the particles which are being processed every frame.
    pub pending_particles:  ParticlePender,             // Struct containing the particles to be added on the next frame; Also handles id assignment
    pub deleting_particles: Vec<usize>,                 // Vector array reffering to particle ID's to be searched from `active_particles` and removed.
}

impl ParticleHandler {
    pub fn new() -> Self {
        Self {
            active_particles:   Vec::new(),
            pending_particles:  ParticlePender::new(),
            deleting_particles: Vec::new(),
        }
    }


    pub fn update(&mut self, delta: f64) {

        self.active_particles.iter_mut().for_each(|p| p.update(delta, &mut self.pending_particles, &mut self.deleting_particles));
    }
}

pub struct ParticlePender {
    pub current_id:         usize,
    pub pending_particles:  Vec<BoxedParticle>,     // Vector array containing particles to be added on the next frame.
}

impl ParticlePender {
    pub fn new() -> Self {
        Self { current_id: 0, pending_particles: Vec::new() }
    }
    pub fn add_particle(&mut self, particle: BoxedParticle) {
        let mut new_particle = particle;
        new_particle.set_id(self.current_id);
        self.pending_particles.push(new_particle);
        self.current_id += 1;
    }
}

