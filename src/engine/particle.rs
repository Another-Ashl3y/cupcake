use crate::engine::material::Material;
use super::manager::ParticlePender;
use super::vector2d::Vector2D;

pub trait Particle {
    fn update(&mut self, delta: f64, pending_queue: &mut ParticlePender, deletion_queue: &mut Vec<usize>);

    fn id(&self) -> usize;

    fn set_id(&mut self, new_id: usize);

    fn material(&self) -> Material;

    fn get_position(&self) -> Vector2D;
    fn set_position(&mut self);

    fn get_velocity(&self) -> Vector2D;
    fn set_velocity(&mut self);
}

pub struct BasicParticle {
    pub id:         usize,          // Particles should be auto-generated each with a unique id
    pub material:   Material,       // Material of the particle used in physics calculations
    pub position:   Vector2D,       
    pub velocity:   Vector2D,
}

impl BasicParticle {
    pub fn new(id: usize, material: Material, position: Vector2D, velocity: Vector2D) -> Self {
        Self { id, material, position, velocity }
    }
    pub fn default(id: usize, position: Vector2D) -> Self {
        Self { id, material: Material::default(), position, velocity: Vector2D::zero() }
    }
    pub fn new_still(id: usize, material: Material, position: Vector2D) -> Self {
        Self { id, material, position, velocity: Vector2D::zero() }
    }
}

