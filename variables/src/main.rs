fn main() {
    // creation
    let a: i16 = 5;

    // mutability
    let mut b: i32 = 5;
    b = 10;

    // shadowing
    let c: i32 = 10;
    let c: i32 = 20;

    println!("c is {c}");

    // scope
    let d: i32 = 30;

    {
        let d: i32 = 40;
        println!("inner d is: {d}")
    }

    println!("d is: {d}")
}
