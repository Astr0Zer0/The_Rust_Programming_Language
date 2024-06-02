fn main() {
    println!("Hello, world!");

    another_function1();

    another_function2(5);

    another_function3(5, 'm');
}

fn another_function1(){
    println!("Another function");
}

fn another_function2(x:i32){
    println!("The value of x is: {x}");
}

fn another_function3(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
