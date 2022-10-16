fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // no semicolon, it means this line return the value
    };

    println!("The value of x is: {}", x); // 5
    println!("The value of y is: {}", y); // 4

    let z = plus_one(5);
    println!("The value of z is: {}", z); // 6
}
