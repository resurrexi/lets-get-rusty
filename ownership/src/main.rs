fn main() {
    // ownership rules
    // 1. each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value is dropped

    let s1 = String::from("Rust"); // heap allocated string

    // let s2 = s1;  // s1 transfers ownership to s2; s1 pointer is destroyed
    let s2 = s1.clone(); // need to clone to have s2 own a copy of the value
    println!("s1 is: {s1}");

    let x = 10;
    let y = x; // 10 is primitive (saved in stack frame) - no need to clone
    println!("x is {x}");
}
