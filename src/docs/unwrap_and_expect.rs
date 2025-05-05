fn unwrap_and_expect() {
    // The `unwrap` method is used to get the value inside an `Option` or `Result` type.
    // If the value is `None` or `Err`, it will panic and terminate the program.
    let some_value: Option<i32> = Some(10);
    let value = some_value.unwrap(); // This will return 10
    println!("Value: {}", value);

    // The `expect` method is similar to `unwrap`, but it allows you to provide a custom error message.
    let none_value: Option<i32> = None;
    let value = none_value.expect("Expected a value, but got None"); // This will panic with the message

    // The `expect` method is also used with `Result` types.
    let result: Result<i32, &str> = Err("An error occurred");
    let value = result.expect("Expected a value, but got an error"); // This will panic with the message
    println!("Value: {}", value);

}
