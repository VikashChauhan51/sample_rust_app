/// print `Hello World` message on terminal/console window.
pub fn msg() {
    let result: i32 = sum(3, 5);
    println!("Hello, world!");
    println!("the sum of two numbers x and y is:{result}")
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
