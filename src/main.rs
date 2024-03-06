mod hello_world;
mod shape;
mod calculator;

use shape::square::Square;
use shape::circle::Circle;
use calculator::calculate_area;

fn main() {
    hello_world::msg();

    let square = Square { side: 5.0 };
    let circle = Circle { radius: 3.0 };

    println!("Square area: {}", calculate_area(&square));
    println!("Circle area: {}", calculate_area(&circle));
}
