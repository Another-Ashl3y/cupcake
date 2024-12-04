pub mod manager;
pub mod particle;
pub mod material;
pub mod vector2d;

pub type BoxedParticle = Box<dyn particle::Particle>;
