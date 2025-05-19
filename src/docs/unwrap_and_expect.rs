fn unwrap_and_expect_examples() {
    // Example 1: Unwrapping a Result with Ok
    let ok_result: Result<&str, &str> = Ok("Success!");
    let value = ok_result.unwrap(); // Returns "Success!"
    println!("Unwrapped Result: {}", value);

    // Example 2: Unwrapping a Result with Err (will panic)
    // let err_result: Result<&str, &str> = Err("Failure!");
    // let value = err_result.unwrap(); // Panics: called `Result::unwrap()` on an `Err` value

    // Example 3: Using expect with Option
    let maybe_number: Option<u32> = Some(42);
    let number = maybe_number.expect("Should have a number");
    println!("Expected number: {}", number);

    // Example 4: Using expect with Result
    let ok_result: Result<u32, &str> = Ok(100);
    let number = ok_result.expect("Should have a value");
    println!("Expected result: {}", number);

    // Example 5: Unwrapping nested Options
    let nested: Option<Option<&str>> = Some(Some("Nested!"));
    let inner = nested.unwrap().unwrap();
    println!("Unwrapped nested Option: {}", inner);

    // Example 6: Using unwrap_or to provide a default value
    let none_value: Option<i32> = None;
    let value = none_value.unwrap_or(0);
    println!("Unwrapped or default: {}", value);

    // Example 7: Using unwrap_or_else with Option
    let none_value: Option<&str> = None;
    let value = none_value.unwrap_or_else(|| "Default value");
    println!("Unwrapped or else: {}", value);

    // Example 8: Using expect_err with Result
    let err_result: Result<u32, &str> = Err("Oops!");
    let error = err_result.expect_err("Expected an error");
    println!("Expected error: {}", error);
}
