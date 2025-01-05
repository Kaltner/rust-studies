fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition is false!");
    }

    // Condition need to be strictly bool. Rust will not try to convert non-bool results into bool
    // The code below would error
    // if number {} 

    let y = 6;

    if y % 4 == 0 {
        println!("number is divisible by 4");
    } else if y % 3 == 0 {
        println!("number is divisible by 3");
    } else if y % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is divisible by 1");
    }

    // Using if in a let statement
    let condition = true
    let x = if condition { 5 } else { 6 };

    println!("The value of the number is: {x}") 

    // Remember that blocks of code evaluate to the last expression
    // Something like the code below will error because all the vaalues that have potential to be the result needs to be the same type.
    // let number = if true { 5 } else { "six" }
}
