fn generic_function_strings() {
    // literal string / str -> save in the static memory
    let _nome: &str = "Fabricio";

    // Dynamic string / string -> save in the heap memory
    let mut s1: String = String::new(); // inicializate s1 with a empty string
    let s2: String = "Fabricio".to_string(); // transform "Fabricio" in string
    let s3: String = "Fabricio".to_owned(); // transform "Fabricio" in a type definition by context
    let s4: String = String::from("Fabricio"); // transform "Fabricio" in a string using String::from

    s1 = String::from("Fabricio Lima");

    println!("{s1}"); // -> Fabricio Lima
    println!("{s2}"); // -> Fabricio 
    println!("{s3}"); // -> Fabricio 
    println!("{s4}"); // -> Fabricio 

    // Concatenation of strings
    let s5: String = String::from("Fabricio ");
    let s6: String = String::from("Lima ");
    
    let s7 = s5 + &s6; // s5 is moved to s7, so s5 is no longer valid
    println!("{s7}"); // -> Fabricio Lima

    // Slice of string
    let s8: &str = &s7[0..7]; // slice from 0 to 7 (not inclusive)
    println!("{s8}"); // -> Fabricio

    let s9: &str = &s7[8..12]; // slice from 8 to 12 (not inclusive)
    println!("{s9}"); // -> Lima

    // String length
    let len: usize = s7.len(); // length of the string

    println!("{len}"); // -> 13

    // formatting strings
    let s10: String = format!("{} {}", s7, len); // format the string with s7 and len
    println!("{s10}"); // -> Fabricio Lima 13

    // String with special characters
    let s11: String = String::from("Fabricio Lima\n"); // \n is a special character for new line
    let s12: String = String::from("Fabricio Lima\t"); // \t is a special character for tab

    let s13: String = String::from("Fabricio Lima\\"); // \\ is a special character for backslash
    let s14: String = String::from("Fabricio Lima\""); // \" is a special character for double quote
    let s15: String = String::from("Fabricio Lima\'"); // \' is a special character for single quote
    let s16: String = String::from("Fabricio Lima\r"); // \r is a special character for carriage return
    let s20: String = String::from("Fabricio Lima\0"); // \0 is a special character for null character
    
    
}
