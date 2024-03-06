use crate::shape;
pub fn calculate_area(s: Box<dyn shape::Shape>) -> f64 {
    s.area()
}

