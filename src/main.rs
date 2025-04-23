use std::io;

mod immutable;
mod memory;
mod primitive_types;
mod raii;
mod strings;
mod template_string;
mod variable_const;
mod variable_let;

mod exercises {
    pub mod add_user;
    pub mod bank_account;
    pub mod check_parity;
    pub mod enter_string;
    pub mod imc_calculator;
    pub mod sum_two_numbers;
    pub mod traffic_light;
    pub mod vehicles;
    pub mod fizz_buzz;
}

static EXERCISES: [&str; 10] = [
    "Enter your name",
    "Sum two numbers",
    "Check Parity",
    "Add User",
    "Bank Account",
    "Vehicles",
    "IMC Calculator",
    "Traffic Light",
    "Fizz Buzz",
    "Doing (don't choose)",
];

fn main() {
    println!("I did {} execises", EXERCISES.len());
    println!("What exercise do u wanna see?");

    let mut index = 1;
    for exercise in EXERCISES {
        println!("{index} -> {}", exercise);
        index = index + 1;
    }

    println!("Enter the exercise number: ");

    let mut choose_input: String = String::new();
    io::stdin()
        .read_line(&mut choose_input)
        .expect("Error reading your choose");

    println!("You choose exercise number {}!", choose_input);
    println!("STARTING EXERCISE!!!");
    println!();
    println!();

    let choose_input = choose_input.trim();

    match choose_input {
        "1" => exercises::enter_string::name_input(),
        "2" => exercises::sum_two_numbers::sum_two_numbers(),
        "3" => exercises::check_parity::check_parity(),
        "4" => exercises::add_user::add_user(),
        "5" => exercises::bank_account::bank_account(),
        "6" => exercises::vehicles::vehicles(),
        "7" => exercises::imc_calculator::imc_calculator(),
        "8" => exercises::traffic_light::traffic_light(),
        "9" => exercises::fizz_buzz::fizz_buzz(),
        _ => println!("Invalid exercise number!"),
    }
}
