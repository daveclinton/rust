fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // Constants are not allowed to change
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing is decalring a new variable with same name as previous varible

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let mut spaces = "  ";
    // spaces = spaces.len();
}
