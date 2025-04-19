use std::io;

pub fn check_parity() {
    let mut number: String = String::new();
    println!("Enter the number:");

    io::stdin()
        .read_line(&mut number)
        .expect("Error reading the number!");

    let number: i32 = number.trim().parse().expect("Please enter a valid number!");

    if number % 2 == 0 {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }
}
