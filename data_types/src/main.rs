use std::io;

fn main() {
    //  Scalar Types (Integers, floating-point numbers, Booleans, and characterss)
    // Integer Types
    // learnt about signed and unsigned bits
    // i means signed and u means unsigned (signed can hold both positive and negative values, while unsigned can only hold positive values)
    // 8-bit i8 u8

    let x: i8 = 36; //signed  8 bit
    println!("{x}");

    let x: u16 = 450;
    println!("{x}");

    // Floating Points
    let new = 2.0;
    let new: f32 = 3.0;

    println!("{new}");

    // Numeric Operations
    // addition
    let sum = 5 + 10;
    // substraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    //  Boolean Type

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("{t} {f}");

    // Character Type

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c} {z} {heart_eyed_cat}");

    // Compound Types
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup; // destructuring using pattern matching

    println!("{a} {b} {c}");

    let gone: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = gone.0;
    let six_point_four = gone.1;
    let one = gone.2;
    println!("zero: {five_hundred} one: {six_point_four} two:{one}");

    // Array Type
    let states = [1, 2, 3, 4, 5];
    let states: [i32; 5] = [1, 2, 3, 4, 5]; // Explicit type annotation

    let right = [3; 5]; // contain the same value of each element

    println!("right: {:?}", right); // use debug formating

    let first_index = states[0];
    println!("{first_index}");

    // Invalid Array Element Access
    let invalid_array = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = invalid_array[index];

    println!("The value of the element at index {index} is: {element}");
}
