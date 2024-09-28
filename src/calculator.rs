use std::io;

pub fn run_calculator() {
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut operator = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut first_number).expect("Failed to read first number");

    println!("Enter an operator (+, -, *, /):");
    io::stdin().read_line(&mut operator).expect("Failed to read operator");

    println!("Enter the second number:");
    io::stdin().read_line(&mut second_number).expect("Failed to read second number");

    let first_number: f64 = first_number.trim().parse().expect("please enter a valid number");
    let second_number: f64 = second_number.trim().parse().expect("please enter a valid number");

    let result = match operator.trim() {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => first_number / second_number,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("The result is: {}", result);
}
