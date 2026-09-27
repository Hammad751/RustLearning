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
    println!("sum = {}, \nsum = {}, with return statement \nmul = {}, \ndiv = {}", sum, sum, multi, divi);

    // Expression
    // An expression is a piece of code that evaluates to a value. 
    // In Rust, expressions do not include ending semicolons. 
    // If you add a semicolon at the end of an expression, it becomes a statement, which does not return a value.
    // Calling a function is an expression
    // calling a macro is an expression
    // calling a block is an expression

    let y = {
        let x = 3;
        x + 1 // This is an expression, no semicolon
    };
    println!("y = {}", y);
}