// This is a simple Rust program that prints "Hello, world!" to the console.
// You can run this program using `cargo run` in the terminal.
// Make sure you have Rust installed and set up correctly.
fn main() {
    
    let mut x = 10; // This line declares a mutable variable x and initializes it with the value 10

    println!("x is a {}", x); // This line prints the value of x to the console

    x = 20;

    println!("x is now a {}", x); // This line prints the updated value of x to the console
    // The following lines demonstrate various data types in Rust
    main_data_types();
    // The following lines demonstrate basic arithmetic operations in Rust
    main_operations();
}

fn integer_data_types() {
    let a: i32 = 10; // 32-bit signed integer
    let b: u32 = 20; // 32-bit unsigned integer
    let c: i64 = 30; // 64-bit signed integer
    let d: u64 = 40; // 64-bit unsigned integer

    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}
fn floating_point_data_types() {
    let e: f32 = 3.14; // 32-bit floating point
    let f: f64 = 2.718281828459045; // 64-bit floating point

    println!("e: {}, f: {}", e, f);
}
fn boolean_data_type() {
    let is_rust_fun: bool = true; // Boolean type

    println!("Is Rust fun? {}", is_rust_fun);
}
fn character_data_type() {
    let letter: char = 'R'; // Character type

    println!("The first letter of Rust is {}", letter);
}
fn string_data_type() {
    let greeting: &str = "Hello, Rust!"; // String slice type

    println!("{}", greeting);
}

fn adition(a: i32, b: i32) -> i32 {
    a + b // This function returns the sum of two integers
}
fn subtraction(a: i32, b: i32) -> i32 {
    a - b // This function returns the difference of two integers
}
fn multiplication(a: i32, b: i32) -> i32 {
    a * b // This function returns the product of two integers
}
fn division(a: i32, b: i32) -> f64 {
    if b == 0 {
        panic!("Cannot divide by zero!"); // This will panic if b is zero
    }
    a as f64 / b as f64 // This function returns the quotient of two integers as a floating point number
}
fn modulus(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot perform modulus by zero!"); // This will panic if b is zero
    }
    a % b // This function returns the remainder of the division of two integers
}

fn main_data_types() {
    integer_data_types();
    floating_point_data_types();
    boolean_data_type();
    character_data_type();
    string_data_type();
}
fn main_operations() {
    let x = 10;
    let y = 5;

    println!("Addition: {}", adition(x, y));
    println!("Subtraction: {}", subtraction(x, y));
    println!("Multiplication: {}", multiplication(x, y));
    println!("Division: {}", division(x, y));
    println!("Modulus: {}", modulus(x, y));
}