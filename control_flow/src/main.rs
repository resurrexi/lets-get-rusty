fn main() {
    // if/else
    let a: i32 = 5;

    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    let b: i32 = if a > 5 { 1 } else { -1 };

    // loop
    'outer: loop {
        println!("loop forever");
        loop {
            break 'outer;
        }
    }

    let x: i32 = loop {
        break 5;
    };

    // while loop
    let mut a: i32 = 0;

    while a < 5 {
        println!("a is {a}");
        a += 1;
    }

    // for loop
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for element in a {
        println!("{element}");
    }
}
