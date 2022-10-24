use ch02_concepts::{Rectangle, Shape};
fn main() {
    let rec = Rectangle { a: 10.0, b: 5.0 };
    let shape = Shape::Rec(rec);
    println!("area is: {}", shape.area());
}