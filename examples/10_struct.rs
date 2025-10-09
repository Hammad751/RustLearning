#![allow(unused)]

#[derive(Debug)]

// There are various ways to define the struct
struct Point{
    x: i32,
    y:i32
}

struct Point3D(i32, i32, i32);

struct Empty;

#[derive(Debug)]
struct Circle{
    radius: u32,
    center: Point
}

// Exercise
#[derive(Debug)]
pub struct Account{
    address: String,
    balance: u32,
}


pub fn new_account(address: String) -> Account {
    // todo!();
    Account{
        address,
        balance: 0,
    }

}

fn main(){

    let p = Point{x: 31, y:15};
    println!("{:#?}", p);

    let p1 = Point3D(12,4,3);
    println!("point 3D: ({}, {}, {})", p1.0, p1.1, p1.2);

    let empty = Empty;

    let circle = Circle{
        radius: 32,
        center: p
    };

    println!("circle:  {:#?}", circle);


    // Exercise

    let data = new_account(String::from("0x1234567890abcdef"));

    println!("My account: {:#?}", data);
    // let my_account = Account{
    //     address: String::from("0x1234567890abcdef"),
    //     balance: 0,
    // };
    // println!("My account: {:#?}", my_account);

}