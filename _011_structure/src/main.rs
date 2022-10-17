#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // impl is an implementation block
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle { // static method
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("bayernmuller@github.com"),
        username: String::from("bayernmuller"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("bayernmuller@gmail.com");

    let user2 = User {
        email: String::from("joshuakimmich@github.com"),
        username: String::from("joshuakimmich"),
        ..user1 // copy the rest of the fields from user1
    };

    println!("{:?}", user1); // User can be printed with {:?} because it implements the Debug trait
    println!("{:#?}", user2); // {:#?} prints the struct in a more readable way


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // nameless struct tuples

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle { width: 10, height: 40 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::square(3);
    println!("rect3 is {:#?}", rect3);

}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand, if the field name and the variable name are the same
        username,
        active: true,
        sign_in_count: 1,
    }
}