// this program is a simple calculator for the command line
//Imports
use std::env::{args, Args};
use std::fmt::format;

fn main() {
    // using the args import to create a struct of the values for the application
    let mut args: Args = args();

    //first argument in the calculation
    let firstArg = args.nth(1).unwrap();
    //the operation taking place
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    //the second argument in the calculation
    let secondArg = args.nth(0).unwrap();

    //parseing args into floats
    let first_num: f32 = firstArg.parse().unwrap();
    let second_num: f32 = secondArg.parse().unwrap();
    let result = calc(operator, first_num, second_num);
    println!("{:?}", output(first_num, operator, second_num, result));
}

//function for the operation of the calculator
fn calc(operator: char, first_num: f32, second_num: f32) -> f32 {
    //using rust pattern match rather than if else to improve read ability
    match operator{
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '*' | 'x' | 'X' => first_num * second_num,
        '/' => first_num / second_num,
        //using panic to properly give an error message for non proper operator
        _ => panic!("Invalid Operator used!")
    }
}

//function to format the output
fn output(first_num: f32, operator: char, second_num: f32, result: f32) -> String
{
    format!("{} {} {} = {}", first_num, operator, second_num, result)
}
