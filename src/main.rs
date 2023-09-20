use std::io;

fn main() {
    loop {
        let mut n = String::new();
        println!("Which fibonacci number are you looking for?");
        io::stdin().read_line(&mut n).expect("Failed to read line!");

        let answer: u32 = match n.trim().parse() {
            Ok(num) => fibonacci(num),
            Err(_) => {
                println!("Please input a number greater than or equal to 0!");
                continue;
            }
        };

        println!("Your fibonacci number is {}!", answer);
    }
}

fn fibonacci(num: u32) -> u32 {
    match num {
        0 => 0,
        1 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}
