pub fn first_word() {
    let mut text = String::new();

    println!("Enter the text");
    std::io::stdin()
        .read_line(&mut text)
        .expect("Error reading the text");

    let mut text_divided = text.split(" ");
    let first_word = text_divided.next().unwrap();

    println!("{}", first_word)
    // for word in text_divided{
    //     println!("{}", word)
    // }
}
