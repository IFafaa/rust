use std::io;

struct User {
    name: String,
    email: String,
    password: String,
    age: u32,
    enabled: bool,
}

pub fn add_user() {
    let user = get_user();

    println!("Name: {}", user.name);
    println!("Email: {}", user.email);
    println!("Password: {}", user.password);
    println!("Age: {}", user.age);
    println!("Enabled: {}", user.enabled);
}

fn get_user() -> User {
    let name = get_name();
    let email = get_email();
    let password = get_password();
    let age = get_age();
    let enabled = true;

    User {
        name,
        email,
        password,
        age,
        enabled,
    }
}

fn get_name() -> String {
    let mut name = String::new();
    println!("Enter your name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Error reading your name!");

    name.trim().to_string()
}

fn get_age() -> u32 {
    let mut age = String::new();
    println!("Enter your age:");

    io::stdin()
        .read_line(&mut age)
        .expect("Error reading your age!");

    let age: u32 = age.trim().parse().expect("Please enter a valid number!");
    age
}

fn get_email() -> String {
    let mut email = String::new();
    println!("Enter your email:");

    io::stdin()
        .read_line(&mut email)
        .expect("Error reading your email!");

    email.trim().to_string()
}

fn get_password() -> String {
    let mut password = String::new();
    println!("Enter your password:");

    io::stdin()
        .read_line(&mut password)
        .expect("Error reading your password!");

    password.trim().to_string()
}
