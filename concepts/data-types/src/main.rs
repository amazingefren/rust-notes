/* NOTES
    Scalar Types (Single Value)
        1. Integers, (Whole Number: 1, 100, 12039, 12409)
        2. Floating-Point Numbers, (Decimals: 1.23012, 0.0123, 12312.1203)
        3. Booleans, (?: True or False)
        4. Characters, (Chars, Strings: a, b, z, e || Hello, World)

    Integers: (u/i)*bitsize* eg i32 = signed 32bit
        signed = neg => pos, unsigned = 0 => pos
        eg: i8 ranges from (-128, 128)
            u8 ranges from (0, 128)
        i/u8
        i/u16
        i/u32 -- default and fast
        i/u64
        i/u128
        isize/usize -- arch depends on 32/64bit, good for indexing collections
        *** Can be written using integer literas:
            Decimal: 98_222
            Hex: 0xff
            Octal: 0o77
            Binary: 0b1111_0000
            Byte(u8 only): b'A'
    Floating:
        f32 = 32 bits
        f64 = 64 bits -- default, newer hardware = same speed as f32, and more precise
    Boolean: 1 bit size
        true
        false
    Numeric Operators:
        + = addition
        - = subtraction
        \* = multiplication
        / = division
        % = remainder
    Character Type:
        '' = Single Char 😻, ℤ, z
        "" = String (aka Char Array) Hello, World, This is a sentence.
        Rust's Char type is four bytes in size for Unicode Scalar Value.
            - This mean it can do more than just ASCII, Chinese, Japanese, Emoji, and more.
    Compount Types:
        Tuple:
            General way of grouping a number of values, Fixed length can't grow or shrink in size.
*/
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // Floats
    let _floating_default = 2.0; // Typeof = f64 chosen by default
    let _floating_assigned: f32 = 2.0; // Typeof = f32 chosen by myself, marginally slightly faster, not worth

    // Bools
    let _bool_true = true; // Recognized of type bool
    let _bool_false: bool = false; // explicit type declaration

    // Tuple
    let tuple_example: (i32, f64, u8) = (-500, 6.4, 1);
    let (tuple_x, tuple_y, tuple_z) = tuple_example;
    println!("x: {}, y: {}, z:{}", tuple_x, tuple_y, tuple_z);
}
