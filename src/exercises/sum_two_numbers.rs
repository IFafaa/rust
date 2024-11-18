use std::io;

pub fn sum_two_numbers() {
    let mut first_number = String::new();
    println!("Enter the first number:");

    io::stdin()
        .read_line(&mut first_number)
        .expect("Error reading the first number!");

    let first_number: i32 = first_number
        .trim()
        .parse()
        .expect("Please enter a valid number!");

    let mut second_number = String::new();
    println!("Enter the second number:");

    io::stdin()
        .read_line(&mut second_number)
        .expect("Error reading the second number!");

    let second_number: i32 = second_number
        .trim()
        .parse()
        .expect("Please enter a valid number!");

    let result = first_number + second_number;

    println!("The numbers sum is: {}", result);
}
