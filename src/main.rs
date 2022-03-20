use core::panic;
use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    // println!("Inner argv before all runs: {:?}", args);
    //
    // You may receive the array value in a variable like this:
    // > let argv = [args.nth(1), args.nth(0), args.nth(0)];
    // In this case argv.get(index).unwrap() -> will resolve to &element (reference)
    // to get and element, you'd have to do, e.g.
    // > let first = argv.get(0).unwrap().expect("Couldn't read first number");
    // use array destructuring instead to get resolved Option values (dereferenced values)

    let [first, operator, second] = [args.nth(1), args.nth(0), args.nth(0)];

    let first = first.expect("Couldn't read first number");
    // println!("Inner argv #1: {:?}", args);

    let operator = operator.expect("Couldn't read operator").chars().next().unwrap();
    // println!("Inner argv #2: {:?}", args);

    let second = second.expect("Couldn't read second number");
    // println!("Inner argv #3: {:?}", args);

    let first_number = first.parse::<f32>().expect("Error parsing first number");
    let second_number = second.parse::<f32>().expect("Error parsing second number");

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
        _ => panic!("Invalid operator used.")
    };
}
