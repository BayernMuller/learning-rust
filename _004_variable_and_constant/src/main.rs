fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 5;
    println!("The value of y is: {}", y);

    y = 6;
    println!("The value of y is: {}", y);

    // Shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);

    // Shadowing with different type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // Overwrite variable with different type
    let mut spaces = "   ";
    // spaces = spaces.len(); 
    // ⬆️ Compile Error

    // Constant
    const PI: f32 = 3.14;
    println!("The value of PI is: {}", PI);

    const CAFE: i32 = 0xCAFE;
    println!("The value of CAFE is: {}", CAFE);
}
