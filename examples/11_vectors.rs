#![allow(unused)]
// Vectors - a resizable array type
// - Vectors are heap allocated
// - Vectors are defined using the Vec<T> type
// - Vectors can store multiple values of the same type
// - Vectors are defined using the vec! macro
// - Vectors can be mutable or immutable
// - Vectors can be indexed using the [] operator
// - Vectors can be iterated using a for loop
// - Vectors can be modified using various methods
// - Vectors can be sliced using the [] operator
// - Vectors can be concatenated using the extend method
// - Vectors can be cloned using the clone method
// - Vectors can be compared using the == operator
// - Vectors can be sorted using the sort method
// - Vectors can be reversed using the reverse method
// - Vectors can be cleared using the clear method
// - Vectors can be checked for emptiness using the is_empty method
// - Vectors can be resized using the resize method
// - Vectors can be truncated using the truncate method
// - Vectors can be converted to slices using the as_slice method
// - Vectors can be converted to arrays using the try_into method
// - Vectors can be converted to strings using the join method
// - Vectors can be created from iterators using the collect method
// - Vectors can be created from arrays using the to_vec method
// - Vectors can be created from slices using the to_vec method
// - Vectors can be created from other vectors using the to_vec method
// - Vectors can be created from other collections using the from method
// - Vectors can be created from other types using the from method
// - Vectors can be created from other types using the into method
// - Vectors can be created from other types using the extend method
// - Vectors can be created from other types using the push method
// - Vectors can be created from other types using the insert method
// - Vectors can be created from other types using the remove method
// - Vectors can be created from other types using the pop method
// - Vectors can be created from other types using the swap_remove method
// - Vectors can be created from other types using the retain method
// - Vectors can be created from other types using the dedup method
// - Vectors can be created from other types using the dedup_by method
// - Vectors can be created from other types using the dedup_by_key method
// - Vectors can be created from other types using the sort_by method
// - Vectors can be created from other types using the sort_by_key method
// - Vectors can be created from other types using the sort_unstable method
// - Vectors can be created from other types using the sort_unstable_by method
// - Vectors can be created from other types using the sort_unstable_by_key method
// - Vectors can be created from other types using the binary_search method
// - Vectors can be created from other types using the binary_search_by method
// - Vectors can be created from other types using the binary_search_by_key method
// - Vectors can be created from other types using the partition method
// - Vectors can be created from other types using the split_off method
// - Vectors can be created from other types using the drain method
// - Vectors can be created from other types using the splice method
// - Vectors can be created from other types using the retain method

fn main(){
    // Creating a vector
    let v0: Vec<i32> = Vec::new(); // empty vector
    let v1 = vec![1, 2, 3, 4, 5]; // vector with initial values
    let mut v2 = Vec::with_capacity(10); // vector with capacity of 10 that makes it mutable

    // Adding elements to a vector
    v2.push(1);
    v2.push(2);
    v2.push(3);

    println!("v0: {:?}", v0);
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    
    // To prevent from crashing the code, we will call the get() method
    // - the get() is the Option of type <i8>
    // - the reference type will pass in the option method

    // Option<&i8>
    // -Index valid -> Some(&value) => this will returns the reference of the value if the Index is valid
    // - Index invalid -> None => This will return None if the index out of bound
    // println!("v[2] {:?}",v.get(2));
    // println!("v[20] {:?}",v.get(20));

    // Accessing elements in a vector
    let first = v1[0]; // using indexing
    let second = v1.get(1); // using get method

    println!("First element of v1: {}", first);
    match second {
        Some(value) => println!("Second element of v1: {}", value),
        None => println!("No second element"),
    }

    // Iterating over a vector
    for i in &v1 {
        println!("Element in v1: {}", i);
    }

    // Modifying elements in a vector
    let mut v3 = vec![10, 20, 30];
    for i in &mut v3 {
        *i += 5; // increment each element by 5
    }
    println!("Modified v3: {:?}", v3);

    // Slicing a vector
    let slice = &v1[1..4]; // slice from index 1 to 3
    println!("Slice of v1: {:?}", slice);

    // Concatenating vectors
    let mut v4 = vec![6, 7, 8];
    v4.extend(&v1); // extend v4 with elements from v1
    println!("Concatenated v4: {:?}", v4);

    // Cloning a vector
    let v5 = v1.clone();
    println!("Cloned v5 from v1: {:?}", v5);

    // Comparing vectors
    let is_equal = v1 == v5;
    println!("v1 and v5 are equal: {}", is_equal);

    // Sorting a vector
    let mut v6 = vec![3, 1, 4, 2];
    v6.sort();
    println!("Sorted v6: {:?}", v6);
    v6.reverse();
    println!("Reversed v6: {:?}", v6);
    v6.clear();
    println!("Cleared v6: {:?}", v6);
    println!("Is v6 empty: {}", v6.is_empty());
    v6.resize(5, 0); // resize v6 to length 5, filling new elements with 0
    println!("Resized v6: {:?}", v6);
    v6.truncate(3); // truncate v6 to length 3
    println!("Truncated v6: {:?}", v6);
    let array = [1, 2, 3];
    let v7 = array.to_vec(); // create vector from array
    println!("Vector v7 from array: {:?}", v7);
    let joined = v1.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
    println!("Joined v1 into string: {}", joined);

    // we can also initialize the length of the vector
    let v: Vec<i8> = vec![0i8; 20];
    println!("V = {:?}", v[2]);

    // Pop - will return an Option with the value in it

    let mut y: Vec<i8> = vec![1,2,3];
    let x: Option<i8> = y.pop();

    println!("X = {:?}", x);

}