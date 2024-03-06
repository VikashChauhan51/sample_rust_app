
pub struct Square {
    pub side: f64,
}
 impl super::Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}