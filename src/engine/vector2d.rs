#[derive(Clone, Copy)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).powf(0.5) // Length of vector
    }
    
    pub fn normalize(&mut self) {
        // Store magnitude so it isn't recalculated every time it is used.
        let mag = self.mag();
        // Prevent zero division error caused by 0 magnitude.
        if !(mag != 0.0) {
            self.x /= mag;
            self.y /= mag;
        }
    }

    pub fn normalized(&self) -> Vector2D {
        // Store magnitude so it isn't recalculated every time it is used.
        let mag = self.mag();
        // Prevent zero division error caused by 0 magnitude.
        if !(mag != 0.0) {
            Vector2D::new(self.x/mag, self.y/mag)
        } else {
            self.clone()    // 0 Vector
        }
    }
}


// +-*/
impl Vector2D {

    pub fn add(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
    pub fn addf(&mut self, other: f64) {
        self.x += other;
        self.y += other;
    }
    pub fn sub(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
    pub fn subf(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
    }

    pub fn mul(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
    pub fn mulf(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }
    pub fn div(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
    pub fn divf(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
    }

    // Repeats of the other ones that return the result instead of becoming it
    #[allow(non_snake_case)]
    pub fn Add(left: Self, right: Self) -> Vector2D {
        Vector2D::new(left.x+right.x, left.y+right.y)
    }
    #[allow(non_snake_case)]
    pub fn Addf(left: Self, right: f64) -> Vector2D {
        Vector2D::new(left.x+right, left.y+right)
    }
    #[allow(non_snake_case)]
    pub fn Sub(left: Self, right: Self) -> Vector2D {
        Vector2D::new(left.x-right.x, left.y-right.y)
    }
    #[allow(non_snake_case)]
    pub fn Subf(left: Self, right: f64) -> Vector2D {
        Vector2D::new(left.x-right, left.y-right)
    }
    #[allow(non_snake_case)]
    pub fn Mul(left: Self, right: Self) -> Vector2D {
        Vector2D::new(left.x*right.x, left.y*right.y)
    }
    #[allow(non_snake_case)]
    pub fn Mulf(left: Self, right: f64) -> Vector2D {
        Vector2D::new(left.x*right, left.y*right)
    }
    #[allow(non_snake_case)]
    pub fn Div(left: Self, right: Self) -> Vector2D {
        Vector2D::new(left.x/right.x, left.y/right.y)
    }
    #[allow(non_snake_case)]
    pub fn Divf(left: Self, right: f64) -> Vector2D {
        Vector2D::new(left.x/right, left.y/right)
    }

}