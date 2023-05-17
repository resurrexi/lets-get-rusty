fn main() {
    // boolean
    let b1: bool = true;

    // unsigned integers
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i5: u128 = 1;

    // signed integers
    let i6: i8 = 1;
    let i7: i16 = 1;
    let i8: i32 = 1;
    let i9: i64 = 1;
    let i10: i128 = 1;

    // floats
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform-specific integers
    let p1: usize = 1;
    let p2: isize = 1;

    // chars, string slices, and string
    let c1: char = 'c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");

    // arrays
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let i1: i32 = a1[4]; // 0-indexed

    // tuples
    let t1: (i32, i32, i32) = (1, 2, 3);
    let t2: (i32, f64, &str) = (5, 5.0, "5");

    let s1: &str = t2.2;
    let (i1, f1, s1) = t2;

    let unit: () = (); // useful for functions that return nothing

    // type aliasing
    type age = u8;
    let a1: age = 57;
}
