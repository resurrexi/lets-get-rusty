fn main() {
    // ownership rules
    // 1. each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value is dropped

    let s1 = String::from("Rust"); // heap allocated string
    print_string(s1.clone());
    let s3 = generate_string();

    // let s2 = s1;  // s1 transfers ownership to s2; s1 pointer is destroyed
    let s2 = s1.clone(); // need to clone to have s2 own a copy of the value
    let s4 = add_to_string(s2);
    println!("s1 is: {s1}");
    println!("s3 is: {s3}");
    println!("s4 is: {s4}");

    let x = 10;
    let y = x; // 10 is primitive (saved in stack frame) - no need to clone
    print_integer(x);
    println!("x is {x}");
    println!("y is {y}");
}

fn print_integer(i: i32) {
    println!("i is: {i}");
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}

fn generate_string() -> String {
    String::from("Ferris")
}

fn print_string(p1: String) {
    println!("{p1}");
} // p1 is dropped
