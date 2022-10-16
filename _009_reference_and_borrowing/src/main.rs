fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    println!("s1 is still valid here!");

    let mut s2 = String::from("Bayern munich");
    change(&mut s2); // mutable reference
    println!("{}", s2);

    let mut s3 = String::from("Bayern munich");
    let r1 = &mut s3; // mutable reference
    let r2 = &mut s3; // mutable reference

    /*
    println!("{}, {}", r1, r2); // error: cannot borrow `s3` as mutable more than once at a time
    */

    let mut s4 = String::from("Bayern munich");
    let r3 = &s4; // immutable reference
    let r4 = &mut s4; // mutable reference
    let r5 = &s4; // immutable reference

    // immutable references can be used after mutable references

    /*
    println!("{}, {}", r3, r4); // error: cannot borrow `s4` as mutable because it is also borrowed as immutable

    While user is using immutable reference, mutable reference should not be used.
    */
    
    let reference = dangle();
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("is the best team in the world. other teams are just trash. Specifically, Liverpool is fucking trash");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
    // ERROR: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
}