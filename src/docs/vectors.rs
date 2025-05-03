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

    vector_numbers.sort(); // sort the vector in ascending order
                           //vector_numbers -> [1,3,4,5,10]

    vector_numbers.reverse(); // reverse the order of the vector
                              //vector_numbers -> [10,5,4,3,1]

    vector_numbers.clear(); // remove all elements from the vector
                            //vector_numbers -> []

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

    //we can change the value of all elements in the vector:
    for i in 0..vector_numbers.len() {
        //we can also use the for loop to iterate over the vector:
        vector_numbers[i] = vector_numbers[i] * 2;
    }
    //another way
    for i in vector_numbers.iter_mut() {
        *i = *i * 2; // access elements by index
    }

    //we can save other type values in the vector:
    struct Person {
        name: String,
        age: u32,
    }
    let mut vector_people: Vec<Person> = Vec::new(); // create an empty vector of Person structs
    vector_people.push(Person {
        name: String::from("John"),
        age: 30,
    });
    vector_people.push(Person {
        name: String::from("Jane"),
        age: 25,
    });
    vector_people.push(Person {
        name: String::from("Bob"),
        age: 40,
    });

    {
        for person in &vector_people {
            println!("Name: {}, Age: {}", person.name, person.age); // iterate over the vector of structs
        }
        //other way
        for i in 0..vector_people.len() {
            println!(
                "Name: {}, Age: {}",
                vector_people[i].name, vector_people[i].age
            ); // access elements by index
        }
    }
}
