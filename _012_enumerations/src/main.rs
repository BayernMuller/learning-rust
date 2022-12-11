enum IpAddrKindV1 {
    V4,
    V6,
}

enum IpAddrKindV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route_v1(ip_kind: IpAddrKindV1)  {
    match ip_kind {
        IpAddrKindV1::V4 => println!("this is V4"),
        IpAddrKindV1::V6 => println!("this is V6"),
    }
}

fn route_v2(ip_kind: IpAddrKindV2) {
    println!("route_v2's ip_kind has own ip address");
    match ip_kind {
        IpAddrKindV2::V4(a, b, c, d) => println!("V4: {}.{}.{}.{}", a, b, c, d),
        IpAddrKindV2::V6(addr) => println!("V6: {}", addr),
    }
}

enum Meesaage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Meesaage {
    fn call(&self) {
        match self {
            Meesaage::Quit => println!("Quit"),
            Meesaage::Move { x, y } => println!("Move: x={}, y={}", x, y),
            Meesaage::Write(s) => println!("Write: {}", s),
            Meesaage::ChangeColor(r, g, b) => println!("ChangeColor: r={}, g={}, b={}", r, g, b),
        }
    }
}

fn x_devide_y(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    match x { // check x is None or not
        Some(x) => match y { // check y is None or not
            Some(y) => match y { // check y is zero or not
                0 => None, // user try to divide by zero
                _ => Some(x / y), // user try to divide by non-zero
            },
            None => None,
        },
        None => None,
    }
}

fn main() {
    let four = IpAddrKindV1::V4;
    let six = IpAddrKindV1::V6;

    route_v1(four);
    route_v1(six);

    //println!("{}", four);
    //println!("{}", six);
    // error[E0277]: `IpAddrKind` doesn't implement `std::fmt::Display`
    // In rust, you can't print an enum directly because it's not int type like C/C++. 

    let home = IpAddrKindV2::V4(127, 0, 0, 1);
    let loopback = IpAddrKindV2::V6(String::from("::1"));

    route_v2(home);
    route_v2(loopback);

    let m1 = Meesaage::Quit;
    let m2 = Meesaage::Move { x: 10, y: 20 };
    let m3 = Meesaage::Write(String::from("hello"));
    let m4 = Meesaage::ChangeColor(255, 0, 0);

    m1.call();
    m2.call();
    m3.call();
    m4.call();

    if let Meesaage::Write(s) = m3 {
        println!("m3 is Write: {}", s);
    }
    else { // m3 is not Write
        println!("m3 is not Write");
    }

    let x = Some(10);
    let y = Some(5);

    println!("x / y = {:?}", x_devide_y(x, y));
    println!("x / None = {:?}", x_devide_y(x, None));
    println!("None / y = {:?}", x_devide_y(None, y));
    println!("None / None = {:?}", x_devide_y(None, None));
    println!("x / 0 = {:?}", x_devide_y(x, Some(0)));

}
