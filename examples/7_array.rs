#![allow(unused)]

// Array - is the collection of elements with known length at compile time
// - the size of the array must be known at compile time
// Slice - is the collection of elements with un-known length at compile time
// - the reason for un-known length is that we can the slice the array inside the program
// - the length of the resulting slice is not known at compile time
// - it only knwon at execution time


fn zeros(){
    let arr: [u32; 10] = [0; 10];
    println!("Array = {:?}", arr);
}

// fn first_3(){
//     let arr: [u32; 7] = [1,2,3,4,5,6,7];
//     let slice: &[u32] = &arr[..3];
//     println!("Slice = {:?}", slice);
// }

pub fn first_3(s: &[u32]) -> &[u32] {
    &s[..3]
}

pub fn last_3(s: &[u32]) -> &[u32] {
    &s[4..]
}

fn main(){
    // Array
    let arr: [u32; 3] = [1,2,3];
    println!("arr = {:?}", arr[0]);

    // Write
    let mut arr: [u32; 3] = [1,2,3];
    arr[1] = 34;
    println!("arr = {:?}", arr[1]);

    // There is another way to write the array with same value

    let arr: [u32; 10] = [2; 10];
    println!("Array = {:?}", arr);


    // Slice
    let nums:[i32; 10] = [-1,1,-2,2,-3,3,-4,4,-5,5];
    // slicing the first 3 indexes from 0-2 index excluding the 3rd index
    // - the first index in not necessary if the index is starting from 0 index
    // - First 1
    let s: &[i32] = &nums[..3];
    
    // this silce the last 3 indexes in the same way
    // - Last
    let s: &[i32] = &nums[7..];
    
    // this will split the array from index 3 to 7 (excluded)
    // Middle
    let s: &[i32] = &nums[3..7];

    println!("Mid : {:?}", s);


    // Exercises

    let array: [u32; 7] = [1,2,3,4,5,6,7];

    zeros();

    println!("{:?}",first_3(&array));
    println!("{:?}",last_3(&array));

}