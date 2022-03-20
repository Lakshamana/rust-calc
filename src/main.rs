use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    // println!("Inner argv before all runs: {:?}", args);
    let first = args.nth(1).unwrap();
    // println!("Inner argv #1: {:?}", args);

    let operator = args.nth(0).unwrap().chars().next().unwrap();
    // println!("Inner argv #2: {:?}", args);

    let second = args.nth(0).unwrap();
    // println!("Inner argv #3: {:?}", args);

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let result = operate(operator, first_number, second_number);
    println!("{}", output(operator, first_number, second_number, result));
}

fn output(operator: char, first_number: f32, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    // You may write all of this as if/else-if/else chains

    // if operator == '+' {
    //     return first_number + second_number;
    // } else if operator == '-' {
    //     // will always return the first expression even if 'return' is not implicit
    //     first_number - second_number
    // } else if operator == '*' {
    //     first_number * second_number
    // } else if operator == '/' {
    //     first_number / second_number
    // } else {
    //     0.0
    // }

    // Recommended way, would be, however
    return match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator.")
    };
}
