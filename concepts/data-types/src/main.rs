/* NOTES
    Scalar Types (Single Value)
        1. Integers, (Whole Number: 1, 100, 12039, 12409)
        2. Floating-Point Numbers, (Decimals: 1.23012, 0.0123, 12312.1203)
        3. Booleans, (?: True or False)
        4. Characters, (Single Letters (1 bit in memory): a, b, z, e)

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
*/
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    //Floats
    let floatingDefault = 2.0; // Typeof = f64 chosen by default
    let floatingAssigned: f32 = 2.0; // Typeof = f32 chosen by myself, marginally slightly faster, not worth

    //Bools
    let boolTrue = true; // Recognized of type bool
    let boolFalse: bool = false; // explicit type declaration
}
