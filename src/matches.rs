fn matches() {
    // match is a keyword in Rust, used for pattern matching
    let number = 5;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Not a number between 1 and 5"), // _ is a catch-all pattern
    }

    // match can also be used with ranges
    let number = 7;
    match number {
        1..=5 => println!("Between 1 and 5"),
        6..=10 => println!("Between 6 and 10"),
        _ => println!("Not in the range"),
    }

    // match can also be used with enums
    enum Color {
        Red,
        Green,
        Blue,
    }
    let color = Color::Green;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    // match can also be used with tuples
    let tuple = (1, 2);
    match tuple {
        (1, 2) => println!("Tuple is (1, 2)"),
        (3, 4) => println!("Tuple is (3, 4)"),
        _ => println!("Tuple is something else"),
    }

    // match can also be used with structs
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1, y: 2 };
    match point {
        Point { x: 1, y: 2 } => println!("Point is (1, 2)"),
        Point { x: 3, y: 4 } => println!("Point is (3, 4)"),
        _ => println!("Point is something else"),
    }

    // match can also be used with references
    let number = &5;
    match number {
        &1 => println!("One"),
        &2 => println!("Two"),
        &3 => println!("Three"),
        &4 => println!("Four"),
        &5 => println!("Five"),
        _ => println!("Not a number between 1 and 5"), // _ is a catch-all pattern
    }

    // match can also be used with if guards
    let number = 5;
    match number {
        n if n < 0 => println!("Negative"),
        n if n == 0 => println!("Zero"),
        n if n > 0 => println!("Positive"),
        _ => println!("Not a number"),
    }

    // match can also be used with destructuring
    let tuple = (1, 2);
    match tuple {
        (x, y) if x == y => println!("Tuple is (x, x)"),
        (x, y) => println!("Tuple is ({}, {})", x, y),
    }

    // match can also be used with enums with data
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
    }
    let shape = Shape::Circle(2.0);
    match shape {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Rectangle(width, height) => {
            println!("Rectangle with width {} and height {}", width, height)
        }
    }

    // match can also be used with Option types
    let some_number: Option<i32> = Some(5);
    match some_number {
        Some(n) if n < 0 => println!("Negative"),
        Some(n) if n == 0 => println!("Zero"),
        Some(n) if n > 0 => println!("Positive"),
        Some(_) => println!("Some number, but not handled specifically"),
        None => println!("No number"),
    }

    // match can also be used with Result types
    let result: Result<i32, &str> = Ok(5);
    match result {
        Ok(n) if n < 0 => println!("Negative"),
        Ok(n) if n == 0 => println!("Zero"),
        Ok(n) if n > 0 => println!("Positive"),
        Ok(_) => println!("Some Ok value not specifically handled"),
        Err(e) => println!("Error: {}", e),
    }

    // match can also use other match expressions
    let number = 5;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        other => println!("Not a number between 1 and 5: {}", other), // other is a catch-all pattern
    }

    // match can also create new blocks
    let number = 5;
    match number {
        1 => {
            println!("One");
            println!("This is a new block");
        }
        2 => {
            println!("Two");
            println!("This is a new block");
        }
        _ => {
            println!("Not a number between 1 and 2");
            println!("This is a new block");
        }
    }
}
