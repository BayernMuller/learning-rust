fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut v2  = Vec::new();
    v2.push(1);
    v2.push(2);

    let third_element = &v[2];
    let optional_third_element = v.get(2);

    println!("The third element is {}", third_element);
    // the type of third_element is &i32

    println!("The third element is {:?}", optional_third_element);
    // the type of optional_third_element is Option<&i32>

    // it means getting value with get() method is safe
    
    let borrow = &v2[0];
    v2.push(3); // this will cause error because v2 is borrowed

    // println!("The first element is {}", borrow);
    // * if above line is uncommented, it will cause error
    // * rust ignore unused variable, so it will not cause error if it is commented

    println!("iterating over vector");
    for i in &v2 {
        println!("i: {}", i);
    }

    println!("iterating over vector and multiplying each element by 50");
    for i in &mut v2 {
        *i *= 50;
        println!("i: {}", i);
    }

    // enumerations allow to store different types in vector
    enum Variant {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Variant::Int(3),
        Variant::Text(String::from("blue")),
        Variant::Float(10.12),
    ];

    for i in &row {
        match i {
            Variant::Int(val) => println!("int: {}", val),
            Variant::Float(val) => println!("float: {}", val),
            Variant::Text(val) => println!("text: {}", val),
        }
    }

    

} // v1 and v2 go out of scope and are freed here, also drop their elements
