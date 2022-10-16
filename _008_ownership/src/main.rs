fn main() {
    // rule of ownership
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let s1 = String::from("Bayern Munich");
    let s2 = s1;

    /*
    println!("s1: {}", s1); // error[E0382]: borrow of moved value: `s1`

    Now, s1 is no longer vaild. because s1's internal pointer is copied to s2 and s1 is dropped.

    Jayden:
        This is similar to C++'s move semantics. 
        I surprised that the default = operator is c++ move constructor in Rust. 😂
    */
    
    println!("s2: {}", s2); // Bayern Munich

    let mut s1 = String::from("Bayern Munich");
    let s2 = s1.clone();
    // this is deep copy.
    s1.push_str(" is the best team in the world");

    println!("s1: {}", s1); // Bayern Munich
    println!("s2: {}", s2); // Bayern Munich
    
    let string = String::from("Hello");

    takes_ownership(string); // string's value moves into the function...
                            // ... and so is no longer valid here
    /*
    println!("string: {}", string); // error[E0382]: borrow of moved value: `string`
    */
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

