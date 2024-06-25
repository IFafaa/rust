use std::io;

pub fn name_input(){
    let mut name: String = String::new();
    println!("enter ur name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Error reading your name!");
    println!("ur name is: {}", name.trim());
}

