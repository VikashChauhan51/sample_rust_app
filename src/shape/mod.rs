pub trait Shape {
    fn area(&self) -> f64;
}

pub mod square;
pub mod circle;