/*
    In Rust, control flow is managed through various structures, like if, else, while, for, loop, match and if let.
    The if and else structures are used to execute different blocks of code based on certain conditions.
    Similar to other languages, while and for are used for looping over a block of code. The while loop repeats a block of code until the condition is false, and the for loop is used to iterate over a collection of values, such as an array or a range.
    The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop. Rust's match structure, which is similar to switch statements in other languages, is a powerful tool used for pattern matching: it checks through different cases defined by the programmer and executes the block where the match is found.
    The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
*/

fn main() {
    // if and else
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // while loop
    let mut count = 0;
    while count < 5 {
        println!("count: {}", count);
        count += 1;
    }

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // match statement
    let number = 13;
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else"),
    }

    // if let syntax
    let some_option = Some(5);
    if let Some(value) = some_option {
        println!("The value is: {}", value);
    }
}
