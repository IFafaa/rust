fn generic_function_raii() {
    //start scope
    let number = 1;
    {
        //it's possible create a scope inside a other scope
        let number = number + 15;
        println!("{}", number); // -> 16
    }
    println!("{}", number); // -> 1
} //end scope
  //all variables declared in this function, when it finish the RAII drop the variable of the memory

//obs: LET's can't be declared in the project root, they need a scope
