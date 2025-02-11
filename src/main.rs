mod shape;
use shape::{Circle, Rectangle, Shape, Square};

fn main() {
    let square: &mut dyn Shape = &mut Square::new(5.0);
    let circle: &mut dyn Shape = &mut Circle::new(5.0);
    let rectangle: &mut dyn Shape = &mut Rectangle::new(5.0, 10.0);

    println!("Square area: {}", square.area());
    println!("Circle area: {:.2}", circle.area());
    println!("Rectangle area: {}", rectangle.area());

    square.scale(2.5);
    circle.scale(2.5);
    rectangle.scale(2.5);

    println!("Square new  area: {}", square.area());
    println!("Circle new area: {:.2}", circle.area());
    println!("Rectangle new area: {}", rectangle.area());
}
