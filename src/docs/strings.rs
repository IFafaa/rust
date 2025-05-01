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

}
