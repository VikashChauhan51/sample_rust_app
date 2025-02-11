pub struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

}

impl super::Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
    fn name(&self) -> &'static str {
        "Rectangle"
    }
}
