fn main() {
    // Project to test the immutability of variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is now: {x}");

    // Constants
    // Cannot change; Need to have a the type declared; Value cannot be assingned at run time
    const CONSTANT_NUMBER: u32 = 32;

    // Shadowing example
    // Different from the variable being multable, the new assignment needs to use let to reassign its value.

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Data Types
    // Rust is a statically typed language, that means the type of each var needs to be know at compile time.
    // Scalar types
    // Integer: i32, u32
    // isize = Depending on the system architecture. So if the system is 32 bits, that will be the size. Usually used when you want to index a collection
    // Decimals: 90_992
    // Floats: 

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Numeric Operations:
    // + - * / %
    // Booleans
    let t = true;
    let f: bool = false
    
    // Characters:
    let c = 'z'; // Note the single quotes
    let z: char = 'Z';
    let heart_eyed_cat = '😻'; //Unicode is considered a char.

    // Compound Types
    // Tuples - Fixed size, different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    let (tx, ty, tz) = tup2; // Destructuring 

    println!("The value of ty is: {ty}");

    let tup3: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    
    // Array - Fixed size, same type. 
    // Not as flexible as Vectors (They can grow), so I probably want to use vectors if I want the array to behave as other less-typed languages
    let a: [i32, 5] = [1, 2, 3, 4, 5]; //i32, 5 elements

    let months = ["Jan", "Feb", "May", "Apr", "Mar", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    let repeated_nums = [3; 5]; // Array of 5 items of value 3

    let first = a[0];
    let second = a[1];

    //If out of bounds, will result in a runtime error
    // let out_of_bounds = a[5];

    // Functions
    another_function();

    another_function_with_args(5, 6);

    // Statements vs Expressions
    // Statements are instructions that perfome some action
    // Expressions evaluate to a resultant value

    statements_and_expressions() // This is an expression by itself

    // Functions with return values
    println!(five())
}

fn another_function() {
    println!("Another function has been run");
}

fn another_function_with_args(x: i32, y: i32) {
    println!(x + y)
}

fn statements_and_expressions() {
    let y = 6; // Statement. Statements do not return a value, so something such as let x = y = 6 would 

    // Expressions evaluate to a value
    let y = {
        let x = 3;
        x + 1 // This is a expression. Adding a ; to the end of it would turn it into a statement.
    }; // Blocks are expressions

    println!("The value of y is: {y}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}