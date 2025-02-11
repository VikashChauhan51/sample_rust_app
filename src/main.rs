mod shape;
use shape::{Circle, Rectangle, Shape, Square};

fn print_shape_info(sp: &dyn Shape) {
    println!("The {} area: {}", sp.name(), sp.area());
}

fn main() {
    let square: &mut dyn Shape = &mut Square::new(5.0);
    let circle: &mut dyn Shape = &mut Circle::new(5.0);
    let rectangle: &mut dyn Shape = &mut Rectangle::new(5.0, 10.0);

    print_shape_info(square);
    print_shape_info(circle);
    print_shape_info(rectangle);

    square.scale(2.5);
    circle.scale(2.5);
    rectangle.scale(2.5);

    print_shape_info(square);
    print_shape_info(circle);
    print_shape_info(rectangle);
}
