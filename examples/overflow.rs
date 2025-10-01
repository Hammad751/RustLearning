#![allow(unused)]

// OverFlow - doesn't panic when compiled with --release

fn main(){

    let mut x = u128::MAX;
    x += 1;

    println!("u32 MAX: {} ,x:  {}", u32::MAX, x);
}