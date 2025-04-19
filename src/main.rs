use std::io::{self, Read};

mod immutable;
mod memory;
mod primitive_types;
mod raii;
mod strings;
mod template_string;
mod variable_const;
mod variable_let;

mod exercises {
    pub mod check_parity;
    pub mod enter_string;
    pub mod sum_two_numbers;
}

static EXERCISES: [&str; 5] = [
    "Enter your name",
    "Sum two numbers",
    "Check Parity",
    "Doing (don't choose)",
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

    if choose_input == "1" {
        exercises::enter_string::name_input()
    }
    if choose_input == "2" {
        exercises::sum_two_numbers::sum_two_numbers()
    }
    if choose_input == "3" {
        exercises::check_parity::check_parity();
    }
}
