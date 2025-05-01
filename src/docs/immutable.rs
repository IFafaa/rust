fn generic_function_immutable(){
    //every variable declare in rust is ummutable (== readonly in TS)
    //to declare a mutable variable it's used mut
    //example:

    let mut number: i32 = 1;
    number = 7;
    println!("{}", number); // -> 7
}

