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
}
