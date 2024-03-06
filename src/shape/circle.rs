
pub struct Circle {
    pub radius: f64,
}

impl super::Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}