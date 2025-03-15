use core::f64;
use std::env::consts;
use std::f32::consts::PI;

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn show_shape(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(x) => f64::consts::PI * x * x,
        Shape::Rectangle(x, y) => x * y,
        Shape::Square(x) => x * x,
    }
}

pub fn show_shape_area() {
    let circle = Shape::Square(1.2);
    let area = show_shape(&circle);
    println!("area = {}", area);
}
