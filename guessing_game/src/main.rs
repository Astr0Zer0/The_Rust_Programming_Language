use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess
    .trim()
    .parse()
    .expect("Please Type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You Win!"),
    }
}

//mut makes the variable mutable - allows it to change values
//io.stdin().readline() - allows the user to input data
// . readline() returns a Result type
// Result has 2 possible values Ok and Err
// expect is a function of Result that crashes the program and displays the message inside
// {} - are placeholders and allow you to display the value of variables

//cargo.lock specifies the versions that were used the first time the program was built
//when this project is rebuilt elsewhere it will know wich versions to look for
//unless you specifically update the versioning
//to specifically update u use:
//cargo update

//1..=100 - 1 to 100 included

//cmp return a enumb called Ordering that has three values greater less equal
//we try to match the result to one of those values

//the trim method gets rid of \r\n and whitespaces at the beggining and end
//parse converts a string to another type