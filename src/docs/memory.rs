
fn generic_function_memory() {
    /*
        In rust we have 3 memory types:
    
        STATIC:
            why -> used to save a static variables (know compilation size)
            size -> fixed
            lifetime -> whole program
            cleanup -> when program finish
    
            example:
    */
    static STATIC_VARIABLE: i32 = 1;

    /*
        STACK:
            why -> function arguments, local variables (know compilation size)
            size -> dynamic (top limit)
            lifetime -> function
            cleanup -> when function returns

            example:
    */
    let _n1: i32 = 1; 
    let _n2: i32 = 2; 
    let _n3: i32 = 3; 
    let _n4: i32 = 4; 

    /*
        HEAP:
            why -> values  that live beyound functions, large values, dynamic size values (unknow compilation size)
            size -> dynamic (up to computer)
            lifetime -> DEFINED BY PROGRAMMER OR LANGUAGE
            cleanup -> RAII

            example:
    */
    //let users = get_users(); // idk the lenght of this method return, so we should use a HEAP MEMORY! //temporary example
}
