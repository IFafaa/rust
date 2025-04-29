fn vectors() {
    //VECTORS:
    /*
    Vectors are used to store a list of elements of the same type.
    They are similar to arrays, but they can grow and shrink in size.
    Vectors are implemented as a contiguous growable array type.
    The size of a vector is not fixed at compile time, but it is fixed at runtime.
    Vectors are stored on the heap, while arrays are stored on the stack.
    */
    let mut vector_numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // === let vector_numbers: Vec<i32> = vec![1,2,3,4,5];
                                                            //we can:
    vector_numbers.push(6); // add an element to the end of the vector
                            //vector_numbers -> [1,2,3,4,5,6]

    vector_numbers.pop(); // remove the last element from the vector
                          //vector_numbers -> [1,2,3,4,5]

    vector_numbers.insert(2, 10); // insert an element at a specific index
                                  //vector_numbers -> [1,2,10,3,4,5]

    vector_numbers.remove(1); // remove an element at a specific index
                              //vector_numbers -> [1,10,3,4,5]

    let first_element = vector_numbers[0]; // access an element by index
    println!("First element: {}", first_element);

    if let Some(last_element) = vector_numbers.last() {
        println!("Last element: {}", last_element); // safely access the last element
    }

    for number in &vector_numbers {
        println!("Number: {}", number); // iterate over the vector
    }

    //we can also use the for loop to iterate over the vector:
    for i in 0..vector_numbers.len() {
        println!("Number: {}", vector_numbers[i]); // access elements by index
    }
}
