fn main1() {

    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}

fn main2(){
    let mut count = 0;

    'countingup: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'countingup;
            }
            remaining -= 1;
        }

        count +=1;
    }

    println!("End count = {count}");
}

fn main3(){
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!");
}

fn main4(){
    let a:[i32; 5] = [10,  20,30, 40, 50];

    let  mut index = 0;

    while index < 5 {
          println!("The value is: {}",a[index]);

          index += 1;
    }
}

fn main5(){
    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("the value is: {element}")
    }
}

fn main(){
    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}