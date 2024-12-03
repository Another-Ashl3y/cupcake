use minifb;
use crate::engine::particle::Particle;

pub struct Frame {
    pub window:             minifb::Window,
    pub buffer:             Vec<u32>,

    pub particles:          Particle_handler,
    pub target_fps:         usize,
}

impl Frame {
    pub fn new(name: &str, width: usize, height: usize, window_options: minifb::WindowOptions) -> Self {
        Self {
            window:         minifb::Window::new(name, width, height, window_options).expect("failed to open window"),
            buffer:         vec![0; width*height],
            particles:      Particle_handler::new(),
            target_fps:     60,
        }
    }
    pub fn update(&mut self) {
        self.particles.update();
    }
}


pub struct Particle_handler {
    current_id: usize,
    pub active_particles:   Vec<Box<dyn Particle>>,     // Vector array of the particles which are being processed every frame.
    pub pending_particles:  Vec<Box<dyn Particle>>,     // Vector array containing particles to be added on the next frame.
    pub deleting_particles: Vec<usize>,                 // Vector array reffering to particle ID's to be searched from `active_particles` and removed.
}

impl Particle_handler {
    pub fn new() -> Self {
        Self {
            current_id:         0,
            active_particles:   Vec::new(),
            pending_particles:  Vec::new(),
            deleting_particles: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.active_particles.iter_mut().for_each(|p| p.update(&mut self.pending_particles, &mut self.deleting_particles));
    }
}

