fn main() {
    let mut s = String::new();

    let the_strongest_football_team = "Bayern Munich";
    // the type of the_strongest_football_team is &str

    let mut s2 = the_strongest_football_team.to_string();
    // the type of s2 is String

    let s3 = String::from("바이에른 뮌헨");
    // the type of s3 is String

    // Basically, String type is encoded in UTF-8
    println!("Length of s3: {}", s3.bytes().len());

    s2.push_str(" is the strongest football team in the world");
    
    println!("s2: {}", s2);

    let s4 = String::from("Mia san ");
    let s5 = String::from("Mia");

    let s6 = s4 + &s5;
    // s4 is moved to s6, so we can't use s4 anymore.

    let s7 = String::from("means \"We are who we are\" in german");

    let full_sentence = format!("{}: {} {}", s2, s6, s7);
    // format!() is similar to println!(), but it returns a String instead of printing it.
    println!("{}", full_sentence);

    // let char = full_sentence[0];
    // ERROR! String type doesn't support indexing because it's encoded in UTF-8.

    // convert s3 to vec<u8>
    let vec = s3.clone().into_bytes();
    println!("as String: {}", s3);
    println!("Actually, it's a vec<u8>: {:?}", vec);

    // convert vec<u8> to String
    let s8 = String::from_utf8(vec).unwrap();
    println!("s8: {}", s8);

    for c in s8.chars() {
        println!("{}", c);
    }

}
