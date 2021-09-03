use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn parse_to_float(num: &mut String) -> f32 {
    return num.trim().parse().unwrap();
}

fn parse_to_char(operator: &mut String) -> char {
    return operator.trim().chars().next().unwrap();
}

fn ask_question(question: String, input: &mut String) {
    print!("{}", question);
    read(input);
}

fn main() {
    println!();
    println!();
    println!("------------------------------");
    println!("Welcome into the Calculator. A simple calculator made in Rust");
    println!("-----------------------------");

    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        ask_question(String::from("what is the first number ?: "), &mut num1);
        ask_question(String::from("what is the second number ?: "), &mut num2);
        ask_question(String::from("what is the operator (/+*-) ?: "), &mut operator);

        let num1: f32 = parse_to_float(&mut num1);
        let num2: f32 = parse_to_float(&mut num2);
        let operator: char = parse_to_char(&mut operator);

        let operations = String::from("/+*-");

        if !operations.contains(operator) {
            println!("unknown operator");
            continue;
        }

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => panic!("error in operator")
        };

        println!("the result of {} {} {} = {}", num1, operator, num2, result);

        let mut replay = String::new();
        let yes_or_no = String::from("yn");

        loop {
            print!("play again ? (y/n): ");
            read(&mut replay);

            let replay: char = parse_to_char(&mut replay);
            if !yes_or_no.contains(replay) {
                println!("you must write y or n !");
                continue;
            } else {
                match replay {
                    'y' => break,
                    'n' => return,
                    _ => panic!("error happened...")
                };
            }
        }
    }
}























