use crate::shape;

/// Calculates the area of a shape.
///
/// This function takes a reference to an object that implements the `Shape` trait and calls its `area` method to calculate the area of the shape.
///
/// # Parameters
///
/// * `s`: A reference to an object that implements the `Shape` trait.
///
/// # Returns
///
/// This function returns a `f64` value representing the area of the shape.
///
/// # Examples
///
/// ```
/// use shape::Shape;
/// use shape::square::Square;
///
/// let square = Square { side: 5.0 };
/// let area = calculate_area(&square);
///
/// println!("The area of the square is {}", area);
/// ```
pub fn calculate_area(s: &dyn shape::Shape) -> f64 {
    s.area()
}
