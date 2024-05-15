use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {guess}");
}

//mut makes the variable mutable - allows it to change values
//io.stdin().readline() - allows the user to input data
// . readline() returns a Result type
// Result has 2 possible values Ok and Err
// expect is a function of Result that crashes the program and displays the message inside
// {} - are placeholders and allow you to display the value of variables