#![allow(unused)]

// OverFlow - doesn't panic when compiled with --release

fn main(){

    let mut x = u32::MAX;
    // x += 1;

    let x = u32::checked_add(u32::MAX, 1);
    // let x = u32::checked_add(4, 1);
    println!("checked_add: {:?}", x);

    // println!("u32 MAX: {} ,x:  {}", u32::MAX, x);

    // u32::checked_add - returns None on overflow
    // - it has 2 states: Some(value) or None 
    // - Some(value) when no overflow
    // - None when overflow
    // - we can use match to handle both cases
    // match u32::MAX.checked_add(1) {
    //     Some(v) => println!("checked_add: {}", v),
    //     None => println!("checked_add: overflow"),
    // }

    // u32::wrapping_add - explicitly allow overflow
    let x = u32::wrapping_add(u32::MAX, 1);
    // let x = u32::wrapping_add(4, 1);
    println!("wrapping_add: {:?}", x);

    // Note: These are the 2 methods to handle the value overflow
}