fn structs() {
    // Structs are used to create custom data types
    // They are similar to classes in other programming languages
    // Example:
    struct User {
        name: String,
        age: u32,
    }
    // Creating an instance of the struct
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
    };
    // Accessing the fields of the struct
    println!("User name: {}", user1.name);
    println!("User age: {}", user1.age);
    // Change the value of a field
    let mut user2 = User {
        name: String::from("Bob"),
        age: 25,
    };
    user2.age = 26;
    println!("User2 age: {}", user2.age);
    // Structs can also have methods
    impl User {
        fn greet(&self) {
            println!(
                "Hello, my name is {} and I am {} years old.",
                self.name, self.age
            );
        }
    }
    // Calling the method
    user1.greet();
    user2.greet();
    // Structs can also have associated functions
    impl User {
        fn new(name: String, age: u32) -> User {
            User { name, age }
        }
    }
    // Creating a new instance using the associated function
    let user3 = User::new(String::from("Charlie"), 28);
    println!("User3 name: {}", user3.name);
    println!("User3 age: {}", user3.age);
    // Structs can also be used as function parameters and return types
    fn print_user(user: &User) {
        println!("User name: {}", user.name);
        println!("User age: {}", user.age);
    }
    // Calling the function with a struct instance
    print_user(&user1);
    print_user(&user2);
    print_user(&user3);
}
