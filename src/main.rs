use std::io;

fn main() {
    loop {
        let mut n = String::new();
        println!("Which fibonacci number are you looking for?");
        io::stdin().read_line(&mut n).expect("Failed to read line!");

        let answer: i32 = match n.trim().parse() {
            Ok(num) => match num < 0 {
                true => {
                    println!("Please input a number greater than zero!");
                    continue;
                }
                false => fibonacci(num),
            },
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("Your fibonacci number is {}!", answer);
    }
}

fn fibonacci(num: i32) -> i32 {
    match num {
        0 => 0,
        1 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}
