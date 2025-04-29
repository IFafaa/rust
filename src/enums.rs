fn enums() {
    // Enums are used to create custom data types that can have multiple values
    // They are similar to unions in other programming languages
    // Example:
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    // Creating an instance of the enum
    let direction = Direction::Up;
    // Matching the enum value
    match direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }

    // Enums can also have associated values
    // Example:
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
        Triangle(f64, f64, f64),
    }
    // Another Example:
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // IMPORTANT: OPTION ENUM
    // The Option enum is used to represent a value that can be either Some(value) or None
    // It is used to handle cases where a value may be absent or not applicable
    // Example:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // Creating an instance of the Option enum
    let some_value = Option::Some(5);
    let none_value: Option<i32> = Option::None;
    let string_value = Option::Some(String::from("Hello"));
    let none_string_value: Option<String> = Option::None;

    // try sum some option with i8
    let some_value_2 = Option::Some(10);
    let value_3 = 5;

    // some_value_2 + value_3; // This will not work because Option is not a number

    let result = value_3 + some_value_2.unwrap_or(0);
    println!("Result: {}", result); // This will print 15
}
