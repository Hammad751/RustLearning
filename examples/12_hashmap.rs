#![allow(unused)]

use std::collections::HashMap;
// Hash Map
// - it's another data-type

fn main(){

    let mut scores: HashMap<String, u32> = HashMap::new();   

    scores.insert("red".to_string(),100);
    scores.insert("blue".to_string(),300);

    println!("Scores= {:#?}", scores);

    // to get the value of a specific team, we use the get method
    // - the get method will return the Option as if there is any vlaeus in return, we will get the Some(&val)
    // - or we will get None()

    let score: Option<&u32> =  scores.get("red");

    println!("Red = {:#?}", score);

    let score: Option<&u32> = scores.get("green");
    println!("Gree = {:#?}", score);

    // to update the HashMap
    // - we use the entry() method taking the key as the parameter
    // - and then we use the or_insert() method to insert the value if the key is not present
    // - if the key is present, the value will not be updated
    scores.entry("green".to_string()).or_insert(500);

    // the key "red" is already present, so the value will not be updated
    // - to update the value, we need to use the insert() method.
    scores.entry("red".to_string()).or_insert(700); // will not update the value

    // Now we will create a mutble reference to the value of the key "black" which is not present
    let black_score: &mut u32 = scores.entry("black".to_string()).or_insert(900);
    *black_score += 100; // we can update the value using the mutable reference (dereference)
    println!("black_score= {:#?}", black_score);

    // we can also update the value of a key if we have the mutable reference to the HashMap
    if let Some(blue_score) = scores.get_mut("blue"){
        *blue_score += 50; // we can update the value using the mutable reference (dereference)

        println!("blue_score= {:#?}", blue_score);
        // println!("Scores= {:#?}", scores);
    }
    println!("Scores= {:#?}", scores);
}