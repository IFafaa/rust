use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
struct Square {
    side: f64,
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct RightTriangle {
    base: f64,
    height: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return self.side * self.side;
    }

    fn perimeter(&self) -> f64 {
        return self.side * 4.0;
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return PI * self.radius.powi(2);
    }

    fn perimeter(&self) -> f64 {
        return 2.0 * PI * self.radius;
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> f64 {
        return (self.width * 2.0) + (self.height * 2.0);
    }
}

impl Shape for RightTriangle {
    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }

    fn perimeter(&self) -> f64 {
        let hypotenuse = (self.base.powi(2) + self.height.powi(2)).sqrt();
        return self.base + self.height + hypotenuse;
    }
}

pub fn geometric_calculator() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Square { side: 4.0 }),
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle {
            width: 5.0,
            height: 2.0,
        }),
        Box::new(RightTriangle {
            base: 3.0,
            height: 4.0,
        }),
    ];

    println!("Creating shapes with the following dimensions:");
    println!("Square with side: 4.0");
    println!("Circle with radius: 3.0");
    println!("Rectangle with width: 5.0 and height: 2.0");
    println!("Right Triangle with base: 3.0 and height: 4.0\n");

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape {}:", i + 1);
        println!("Area: {:.2}", shape.area());
        println!("Perimeter: {:.2}\n", shape.perimeter());
    }
}
