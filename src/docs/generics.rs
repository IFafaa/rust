fn generics() {
    // Generics allow us to write code that works with different data types.
    // Instead of duplicating code for each type, we can define functions, structs, enums, and traits
    // that work with generic types.

    // A generic function:
    // This function returns the largest value from a slice
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![1, 3, 5, 2, 4];
    let max_number = largest(&numbers);
    println!("The largest number is: {}", max_number);

    let chars = vec!['a', 'y', 'm', 'z'];
    let max_char = largest(&chars);
    println!("The largest char is: {}", max_char);

    // A generic struct:
    struct Point<T> {
        x: T,
        y: T,
    }

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.1, y: 2.2 };

    println!("Int point: ({}, {})", int_point.x, int_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);

    // We can also have different types for each field:
    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }

    let mixed_point = MixedPoint { x: 5, y: 1.2 };
    println!("Mixed point: ({}, {})", mixed_point.x, mixed_point.y);

    // Generics in Enums:
    // The Option enum is a built-in example that uses generics
    enum MyOption<T> {
        Some(T),
        None,
    }

    let some_number = MyOption::Some(10);
    let some_text = MyOption::Some("hello");

    // Generics with Result enum:
    // Another built-in example that uses generics is Result<T, E>
    // Useful for returning either a value or an error
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
