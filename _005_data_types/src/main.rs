fn main() {
    // integer types
    let a: i8 = 10;         // range: -128 to 127
    let b: i16 = 10;        // range: -32,768 to 32,767
    let c: i32 = 10;        // range: -2,147,483,648 to 2,147,483,647
    let d: i64 = 10;        // range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    let e: u8 = 10;         // range: 0 to 255
    let f: u16 = 10;        // range: 0 to 65,535
    let g: u32 = 10;        // range: 0 to 4,294,967,295
    let h: u64 = 10;        // range: 0 to 18,446,744,073,709,551,615
    let i: isize = 10;      // range: -2^31 to 2^31-1
    let j: usize = 10;      // range: 0 to 2^32-1
                            // depending on the architecture of the computer
                            // in 32-bit architecture, isize and usize are 32-bit

    // floating point types
    let m: f32 = 10.0;      // range: 1.2E-38 to 3.4E+38
    let n: f64 = 10.0;      // range: 2.3E-308 to 1.7E+308
     
    // boolean type
    let o: bool = true;

    // character type
    let p: char = 'a';
    let q: char = '😂';     // unicode character

    // array type
    let r: [i32; 5] = [1, 2, 3, 4, 5];
    r[0] == 1;              // true
    r[1] == 2;              // true
    r[2] == 3;              // true
    r[3] == 4;              // true
    r[4] == 5;              // true

    let s: [i32; 5] = [0; 5];
    s[0] == 0;              // true
    s[1] == 0;              // true
    s[2] == 0;              // true
    s[3] == 0;              // true
    s[4] == 0;              // true

    // string type
    let t: &str = "Hello, world!";

    // tuple type
    let u: (i32, f64, u8) = (500, 6.4, 1);
    u.0 == 500;             // true
    u.1 == 6.4;             // true
    u.2 == 1;               // true

    let (v, w, x) = u;
    v == 500;               // true
    w == 6.4;               // true
    x == 1;                 // true
}