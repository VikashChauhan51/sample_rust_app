
/// The `Square` struct represents a square shape.
/// 
/// # Examples
/// 
/// ```
/// use shape::square::Square;
/// 
/// let square = Square { side: 5.0 };
/// 
/// println!("Square area: {}", square.area());
/// ```
pub struct Square {
    /// The length of a side of the square.
    pub side: f64,
}
 impl super::Shape for Square {
      /// Calculates and returns the area of the square.
    fn area(&self) -> f64 {
        self.side * self.side
    }
}