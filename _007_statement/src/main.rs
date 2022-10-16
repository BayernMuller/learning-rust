fn main() {
    // IF statement
    let number = 3;

    if number < 5 {
        println!("number is less than 5");
    }
    else if number == 5 {
        println!("number is equal to 5");
    } else {
        println!("number is greater than 5");
    }

    /*
    Following code will not compile because the condition must be a bool

    if number {
        println!("number was three");
    }
    */

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    /*
    Following code will not compile because the if and else must have the same type

    let number = if condition {
        5
    } else {
        "six"
    };
    */

    // LOOP statement
    loop {
        println!("again!");
        break;
    }

    // WHILE statement

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    // FOR statement
    for element in [10, 20, 30, 40, 50].iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
