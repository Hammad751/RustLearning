#![allow(unused)]

// Tuple is a collection of values of different types
// Tuples are fixed in size
// Tuples are created using parentheses
// Tuples can be destructured to access individual values

fn return_tuple() -> (bool u32)  // Return many tuples
{
    (true, 1u32)  // These must not be any semicolon for returning the tuple
}

fn main(){
    let t: (bool, char, u32) = (true, 'a', 42);
    let (b, c, n) = t;  // Destructure the tuple into individual variables
    println!("b: {}, c: {}, n: {}", b, c, n);

    let first = t.0;  // Access tuple elements by index
    let second = t.1;
    let third = t.2;
    println!("first: {}, second: {}, third: {}", first, second, third);

    println!("Tuple: {:?}", t);  // Debug '{:?}' print the entire tuple

    let ut = ();  // Unit type, a tuple with zero elements
    println!("Unit type: {:?}", ut);

    // Nested Tuple
    let nt = (('v', 32), ("hammad", 30), (true, 1) );


    // Destructuring the nested Tuple
    println!("nt: {:?}", nt);
    println!("nt: {:?}", nt.0);
    println!("nt: {:?}", nt.1);
    println!("nt: {:?}", nt.2);

    // Finding nested value

    println!("\nFinding nested values\n");
    println!("nt: {:?}", nt.0.1);
    println!("nt: {:?}", nt.1.0);
    println!("nt: {:?}", nt.2.0);
    
    // Partial destructuring

    let (_, m, _) = t;

    println!("Value of M: {:?}", m);

    // return multiple values using a tuple

    let (x,z) = return_many();

    println!(x {}, y{}, x,y;);

}