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
}


pub mod square;

pub mod circle;