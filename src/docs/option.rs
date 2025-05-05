fn option() {
    // Option is an enum that represents a value that can be either Some(T) or None.
    // It is used to handle cases where a value may be absent.
    // The Option type is defined as follows:
    // pub enum Option<T> {
    //     Some(T),
    //     None,
    // }
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(value) => println!("x has a value: {}", value),
        None => println!("x is None"),
    }

    match y {
        Some(value) => println!("y has a value: {}", value),
        None => println!("y is None"),
    }

    // The Option type is often used in functions that may not return a value.
    // For example, the following function returns an Option<i32>:
    fn find_index<T: PartialEq>(vec: &[T], value: T) -> Option<usize> {
        for (index, item) in vec.iter().enumerate() {
            if *item == value {
                return Some(index);
            }
        }
        None
    }
    let vec = vec![1, 2, 3, 4, 5];
    let index = find_index(&vec, 3);
    match index {
        Some(i) => println!("Found at index: {}", i),
        None => println!("Not found"),
    }
    // The Option type is also used in the standard library for functions that may not return a value.
    // For example, the following function returns an Option<&str>:
    fn get_first_word(s: &str) -> Option<&str> {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return Some(&s[0..i]);
            }
        }
        None
    }
    let s = "Hello, world!";
    let first_word = get_first_word(s);
    match first_word {
        Some(word) => println!("First word: {}", word),
        None => println!("No words found"),
    }

    // Using unwrap to extract the value from an Option.
    let z: Option<i32> = Some(10);
    println!("z has a value: {}", z.unwrap_or(0)); // If z is None, it will return 0 instead of panicking.

    // Be cautious when using unwrap, as it will panic if the Option is None.
    let w: Option<i32> = None;
    // Uncommenting the next line will cause a panic:
    // println!("w has a value: {}", w.unwrap());
}
