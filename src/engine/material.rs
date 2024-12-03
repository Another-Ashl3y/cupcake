pub struct Material {
    pub density:                f64,
    pub sticky_data:            StickyData,
    pub has_collision:          bool,
    pub energy_conservation:    f64,        // The multiplier for which the velocity decreases (could also be referred to as passive air resistance)
}

impl Material {
    pub fn new(density: f64, sticky_data: StickyData, has_collision: bool, energy_conservation: f64) -> Self {
        Self { density, sticky_data, has_collision, energy_conservation }
    }
    pub fn default() -> Self {
        Self { density: 1.0, sticky_data: StickyData::default(), has_collision: true, energy_conservation: 1.0 }
    }
}

pub struct StickyData {
    pub sticky:                 bool,       // Glue does stuffs or not
    pub stick_strength:         f64,        // The strength of the glue
    pub preferred_distance:     f64,        // Distance that the paired particle is moved toward
    pub max_distance:           f64,        // Furthest distance that the paired particle will be moved by self
}

impl StickyData {
    pub fn new(sticky: bool, stick_strength: f64, preferred_distance: f64, max_distance: f64) -> Self {
        Self { sticky, stick_strength, preferred_distance, max_distance }
    }
    pub fn default() -> Self {
        Self { sticky: false, stick_strength: 1.0, preferred_distance: 1.0, max_distance: 1.0 }
    }
}


