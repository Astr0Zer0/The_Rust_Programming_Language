use std::io;

fn main() {
    println!("Welcome to the Fahrenheit-Celsius converter.");

    'exit: loop {
        let mut choice = String::new();
        println!("Choose your initial unit: 1-Fahrenheit 2-Celsius 3-quit");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice from terminal");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if choice == 3 {
            break 'exit;
        }

        if choice != 1 && choice != 2 {
            println!("Please choose a valid option (1, 2, or 3).");
            continue;
        }

        let mut input = String::new();

        println!("Enter the temperature to convert:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input from terminal");

        let temperature: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if choice == 1 {
            let celsius = (temperature - 32.0) * 5.0 / 9.0;
            println!("{:.2} Fahrenheit is {:.2} Celsius.", temperature, celsius);
        } else if choice == 2 {
            let fahrenheit = temperature * 9.0 / 5.0 + 32.0;
            println!("{:.2} Celsius is {:.2} Fahrenheit.", temperature, fahrenheit);
        }
    }
}
