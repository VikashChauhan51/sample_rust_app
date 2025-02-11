/// The `Circle` struct represents a circle shape.
///
/// # Examples
///
/// ```
/// use shape::circle::Circle;
///
/// let circle = Circle { radius: 3.0 };
///
/// println!("circle area: {}", circle.area());
/// ```
pub struct Circle {
    /// The radius of the circle.
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}
impl super::Shape for Circle {
    /// Calculates and returns the area of the square.
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }

    fn name(&self) -> &'static str {
        "Circle"
    }
}
