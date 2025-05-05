use std::fs::File;
use std::io::ErrorKind;

pub fn create_file_if_not_exist() {
    let file_path = "src/exercises/helloworld.txt";

    let file = File::open(file_path);
    match file {
        Ok(file) => {
            println!("File already exists! {:?}", file);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found! Creating file...");
                let new_file = File::create(file_path);
                match new_file {
                    Ok(file) => {
                        println!("File created successfully! {:?}", file);
                    }
                    Err(e) => {
                        panic!("Error creating file: {:?}", e);
                    }
                }
            }
            _ => {
                panic!("Error opening file: {:?}", error);
            }
        },
    }
}
