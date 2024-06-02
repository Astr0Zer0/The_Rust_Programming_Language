fn main(){
    let number = 6;

    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    if number != 0 {
        println!("THe number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let number2 = if number == 7 {7} else {1};

    println!("The value of number2 is: {number2}");
}