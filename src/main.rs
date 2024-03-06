mod hello_world;
mod shape;
mod calculator;

use shape::square::Square;
use shape::circle::Circle;

fn main() {
    hello_world::msg();

    let square = Square { side: 5.0 };
    let circle = Circle { radius: 3.0 };

    let square_box: Box<Square> = Box::new(square);
    let circle_box: Box<Circle> = Box::new(circle);

    println!("Square area: {}", calculator::calculate_area(square_box));
    println!("Circle area: {}", calculator::calculate_area(circle_box));
}
