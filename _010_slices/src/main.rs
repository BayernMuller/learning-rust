fn main() {
    let s = String::from("Bayern München");

    let slice = &s[0..6]; // same with &s[..=5];
    println!("{}", slice); // Bayern

    let slice = &s[7..]; // same with &s[7..s.len()];
    println!("{}", slice); // München

    let s = String::from("Die mannschaft");

    let first_word = first_word(&s);
    println!("{}", first_word); // Die

    let first_word = first_word_in_slice(&s[..]);
    println!("{}", first_word); // Die

    let s = "Hello World";
    let first_word = first_word_in_slice(s);
    println!("{}", first_word); // Hello

    // if "first_word"'s parameter is &str, we can pass both &String and &str.

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_in_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}