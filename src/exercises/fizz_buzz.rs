pub fn fizz_buzz() {
    for number in 0..=100 {
        let is_divisible_by_three: bool = number % 3 == 0;
        let is_divisible_by_five: bool = number % 5 == 0;
        if is_divisible_by_three && is_divisible_by_five {
            println!("FizzBuzz");
        } else if is_divisible_by_three {
            println!("Fizz");
        } else if is_divisible_by_five && !is_divisible_by_three {
            println!("Buzz");
        } else {
            println!("{}", number)
        }
    }
}
