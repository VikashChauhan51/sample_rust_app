//! This is a module-level documentation for the `shape` module.
//!
//! This module provides structures and implementations for different shapes.
//!
//! # Examples
//!
//! ```
//! use shape::Shape;
//! use shape::square::Square;
//! use shape::circle::Circle;
//!
//! let square = Square { side: 5.0 };
//! let circle = Circle { radius: 3.0 };
//!
//! println!("Square area: {}", square.area());
//! println!("Circle area: {}", circle.area());
//! ```
/// The `Shape` trait defines a common interface for all shapes.
///  
/// # Examples
///
/// ```
/// use shape::Shape;
/// use shape::square::Square;
///
/// let square = Square { side: 5.0 };
///
/// println!("Square area: {}", square.area());
/// ```
pub trait Shape {
    /// Calculates and returns the area of the shape.
    fn area(&self) -> f64;

    /// Increase the size of shape.
    fn scale(&mut self, factor: f64);

    /// Return shape name.
    fn name(&self) -> &'static str;
}

pub mod circle;
pub mod rectangle;
pub mod square;

pub use self::circle::Circle;
pub use self::rectangle::Rectangle;
pub use self::square::Square;
