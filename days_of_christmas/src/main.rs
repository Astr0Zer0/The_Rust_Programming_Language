fn main() {
    let songDays: [String; 12] = [
        "And partridge in a pear tree".to_string(),
        "Two turtle doves".to_string(),
        "Three French hens".to_string(),
        "Four calling birds".to_string(),
        "Five golden rings".to_string(),
        "Six geese a-laying".to_string(),
        "Seven swans a-swimming".to_string(),
        "Eight maids a-milking".to_string(),
        "Nine ladies dancing".to_string(),
        "Ten lords a-leaping".to_string(),
        "Eleven pipers piping".to_string(),
        "Twelve drummers drumming".to_string(),
    ];

    let mut counter = 1;

    loop {
        if counter == 13 {
            break;
        }

        println!("On the {}th day of Christmas,", counter);
        println!("My true love game to me,");
        for index in (0..counter).rev() {
            if counter == 1 {
                println!("A partridge in a pear tree.")
            } else {
                println!("{}", songDays[index]);
            }
        }

        println!(" ");

        counter += 1;
    }
}
