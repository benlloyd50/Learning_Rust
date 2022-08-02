use std::io;

fn main() {
    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        println!("Input your first number");
        io::stdin()
            .read_line(&mut num1)
            .expect("That's not a number");

        println!("Input your second number");
        io::stdin()
            .read_line(&mut num2)
            .expect("That's not a number");
        
        println!("Operator");
        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read line");
        
        let num1: i32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };
        let num2: i32 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };
        let operator: char = match operator.trim().parse() {
            Ok(op) => op,
            Err(_) => return,
        };

        let ans: i32;
        match operator {
            '+' => ans = num1 + num2,
            '-' => ans = num1 - num2,
            '/' => ans = num1 / num2,
            '*' => ans = num1 * num2,
            _ => return,
        }

        println!("{ans}");
    }
}
