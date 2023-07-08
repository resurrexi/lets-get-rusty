fn main() {
    // tuples
    let rgb_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);

    // tuple structs
    struct Rgb(i32, i32, i32);
    struct Cmyk(i32, i32, i32, i32);

    let color1 = Rgb(255, 106, 0);
    let color2 = Cmyk(0, 58, 100, 0);

    // unit-like structs
    // rarely used; usually used for traits without storing any data
    struct MyStruct;
}
