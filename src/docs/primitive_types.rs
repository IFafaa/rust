fn generic_function_primitive_types() {
    //INT:
    /*
    +-----------+-------+-------+
    | bits | signed | unsigned |
    +-----------+-------+-------+
    | 8    | i8     | u8       |
    | 16   | i16    | u16      |
    | 32   | i32    | u32      |
    | 64   | i64    | u64      |
    | 128  | i128   | u128     |
    | arch | isize  | usize    |
    +-----------+-------+-------+
    signed -> accept negative numbers
    unsigned -> dont't accept negative numbers, just positive
    example:
    */
    let _signed_number: i32 = -20;
    let _unsigned_number: u32 = 40;

    //FLOAT:
    /*
    f64 -> default
    f32
    example:
    */
    let _my_float_number: f64 = 3.14;

    //BOOLEAN:
    /*
    bool -> true and false 😐😐😐
    example:
    */
    let _my_boolean: bool = true;

    //CHAR:
    /*
    Its used to save a one character
    obs: accept emoji
    example:
    */
    let _my_char: char = 'A';
    let _heart_emoji: char = '❤';
    let _chinese_character: char = '你';

    //TUPLE:
    /*
    it's used to save a different variables types in a unique variable
    examples:
    */

    let _tuple_numbers: (i32, i32, i32) = (1, 2, 3); // === let tuple_numbers: (i32, i32, i32) = (1,2,3);

    //we can:
    let _tuple_different_numbers: (f32, i32, u32) = (3.6, -20, 40);

    //we can:
    let mut mutable_tuple: (i32, i32, i32) = (1, 2, 3);
    mutable_tuple.0 = 5;
    //index of tuple
    //mutable_tuple  -> (5,2,3)

    //we can:
    let mut _mutable_tuple2: (i32, i32, i32) = (1, 2, 3);
    _mutable_tuple2 = (4, 5, 6);
    //mutable_tuple  -> (4,5,6)

    //we can:
    let _tuple_deconstruct: (i32, i32, i32) = (1, 2, 3);
    let (_a, _b, _c): (i32, i32, i32) = _tuple_deconstruct;
    //_tuple_deconstruct -> (1,2,3)

    //ARRAY:
    /*
    Array it's used to save a limited data of the same type
    example:
     */
    //type
    let mut array_numbers: [i32; 3] = [1, 2, 3];
    //length
    array_numbers[1] = 7;
    //array_numbers  -> [1,7,3]
}
