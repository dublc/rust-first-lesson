const MY_PI: f64 = std::f64::consts::PI;

pub struct Rectangle {
    pub a: f64,
    pub b: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.a * self.b
    }
}

pub struct Circle {
    r: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        self.r * self.r * MY_PI
    }
}

pub struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    pub fn area(&self) -> f64 {
        let (a, b, c) = (self.a, self.b, self.c);
        let p = (a + b + c) / 2.0;
        (p * (p - a) * (p - b) * (p - c)).sqrt()
    }
}

pub enum Shape {
    Rec(Rectangle),
    Cir(Circle),
    Tri(Triangle),
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.area(),
            Shape::Cir(c) => c.area(),
            Shape::Tri(t) => t.area(),
        }
    }
}