
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
    pub radius: f64,
}

impl super::Shape for Circle {
    /// Calculates and returns the area of the square.
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}