use std::io::stdin;

pub fn get_input() -> usize {
    let mut user_input = String::new();

    println!("Enter number:");

    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().parse().expect("Please type number!")
}
