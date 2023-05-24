fn main() {
    let z: i32 = my_function(22);
    println!("my_function returned: {}", z);
}

fn my_function(x: i32) -> i32 {
    println!("my_function called with: {}", x);
    let y: i32 = 10;
    y
}
