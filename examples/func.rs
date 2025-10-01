#![allow(unused)]

fn add_with_return(x: u32, y: u32) -> (u32){
    return x+y;
}

fn add (x:u32, y:u32) -> u32{
    println!("x = {x}");
    println!("y = {y}");

    x+ y
}

fn mul(x:u32, y: u32) -> u32 {
    x*y
}

fn div(x:u32, y: u32 ) -> u32 {
    x/y
}

fn main(){
    let x = 2;
    let y = 4;

    let sum = add(x, y);
    let sum = add_with_return(x, y);
    let multi = mul(x,y);
    let divi = div(x,y);
    println!("sum = {}, sum = {}, mul = {}, div = {}", sum, sum, multi, divi);
}