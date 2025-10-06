#![allow(unused)]

// Scalar - data types that represents a single value
// - these are:
// - Signed Integers
// - Unsinged Integers
// - Boolean
// - Characters


fn check_char(x: char, y: char) -> bool {
    println!("check_char = {x} {y}", );
    let z: bool = x==y;

    println!("Z = {z}");
    return z;
}

fn main(){
    // Signed Integers
    // Range of values: -2^(n-1) to 2^(n-1)-1
    // -2^(8-1) to 2^(8-1)-1
    // n = number of bits
    // i8, i16, i32, i64, i128, isize (architecture dependent)  
    let i0: i8 = -128; // 8 bits

    // Unsinged Integers
    // Range of values: 0 to 2^n - 1
    // 0 to 2^8 - 1
    // n = number of bits
    // u8, u16, u32, u64, u128, usize (architecture dependent)
    let u0: u8 = 255; // 8 bits

    // Boolean
    // true or false
    let b0: bool = true;
    let b1: bool = false;

    // Characters
    // 4 bytes in size
    // represents a single character
    let c0: char = 'a';
    let c1: char = '😊';
    let c2: char = '中';
    let c3: char = '\n'; // newline character
    let c4: char = '\u{1F600}'; // Unicode scalar value
    let c5: char = std::char::from_u32(0x1F600).unwrap(); // Unicode scalar value from u32

    check_char(c0, c1);
    println!("Characters: {} {} {} {} {} {}", c0, c1, c2, c3, c4, c5);

    // Type Casting
    let i1: i16 = i0 as i16; // casting i8 to i16
    let u1: u16 = u0 as u16; // casting u8 to u16
    let i2: i8 = i1 as i8; // casting i16 to i8
    let u2: u8 = u1 as u8 ; // casting u16 to u8
    let t0: i32 = 1000;
    let t1: u32 = 2000;
    let t2: i8 = t0 as i8; // casting i32 to i8
    let t3: u16 = t1 as u16; // casting u32 to u16
    let t4: u32 = i0 as u32; // casting i8 to u32
    println!("Type Casting: {} {} {} {} {} {} {} {} {}", i1, u1, i2, u2, t0, t1, t2, t3, t4);

    // float values
    let f1: u8 = 5;
    let f2: i8 = -3;

    let f3: f32 = f1 as f32;
    let f4: f32 = f2 as f32;

    let sum: f32 = f3 + f4;

    println!("sum = {sum}");

}
