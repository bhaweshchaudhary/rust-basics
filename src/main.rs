// Import calulator module
mod calculator;

// fn: Defines a function
// main: The entry point of every rust program
// println!: A macro that prints to the console

fn main() {
    println!("============================================================");
    println!("||                    Hello, Rust Program                 ||");
    println!("============================================================");
    println!("hello rust");

    // In Rust, variables are immutable by default. You can make them mutable using the mut keyword.
    // let: Used to declare variables.
    // mut: Makes a variable mutable.

    println!("============================================================");
    println!("||                      Variables                         ||");
    println!("============================================================");

    let x = 5; // immutable
    let mut y = 10; // mutable

    y += 5;

    println!("x: {}, y: {}", x, y);

    println!("============================================================");
    println!("||                    Basic Data Types                    ||");
    println!("============================================================");

    let integer: i32 = 10;
    let float: f64 = 10.34;
    let boolean: bool = true;
    let character = 'a';

    println!("Integer: {}, Float: {}, Boolean: {}, Character: {}", integer, float, boolean, character);

    println!("============================================================");
    println!("||                Conditional Statements                   ||");
    println!("============================================================");

    let number = 10;
    if number % 2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }

    println!("============================================================");
    println!("||                        Loops                            ||");
    println!("============================================================");

    let mut count = 0;

    println!("******************** Infinite Loop ********************");

    loop {
        if count == 5 {
            break;
        }
        println!("Count: {}", count);
        count += 1;
    }

    println!("******************** While Loop ********************");

    while count < 10 {
        println!("While count: {}", count);
        count += 1;
    }

    println!("******************** For Loop ********************");

    for num in 10..15 {
        println!("For num: {}", num);
    }

    println!("============================================================");
    println!("||                    SIMPLE CALCULATOR                    ||");
    println!("============================================================");


    println!("
     ____  _            _       _             
    / ___|| | ___   ___| | __ _| |_ ___  _ __ 
    \\___ \\| |/ _ \\ / __| |/ _` | __/ _ \\| '__|
     ___) | | (_) | (__| | (_| | || (_) | |   
    |____/|_|\\___/ \\___|_|\\__,_|\\__\\___/|_|   
    ");

    calculator::run_calculator();
}
