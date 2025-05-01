use std::io;

mod docs;

mod exercises;
use exercises::EXERCISES;

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
        "10" => exercises::little_calendar::little_calendar(2025),
        "11" => exercises::first_word::first_word(),
        "12" => exercises::rock_paper_scissors::rock_paper_scissors(),
        "13" => exercises::geometric_calculator::geometric_calculator(),
        _ => println!("Invalid exercise number!"),
    }
}
