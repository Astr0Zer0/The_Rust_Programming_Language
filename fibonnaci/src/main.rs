use std::io;

fn main() {
    println!("Welcome to fibonnaci numbers!");

    loop {
        println!("Please input a number or -1 to exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Please a valid choice");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == -1 {
            break;
        }

        println!(
            "The {}th number of the fibonnaci sequence starting at 0 is: {}",
            choice,
            fibonacci(choice)
        );
    }
}

fn fibonacci(x: i32) -> i32 {
    if x <= 1 {
        x
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}
