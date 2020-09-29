/*
    RUST DOES NOT CARE ABOUT FUNCTION DECLARATION ORDER, COOOOOOL MAIN CAN BE ON TOP DEFAULT :D
*/

fn main() {
    // let x = let y = 6;
    println!("Hello, world!");
    test_use_snake();
    another_function(5, "Hello");
    let sample_five = five();
    println!("sample_five: {}", plus_one(sample_five));
}

fn test_use_snake() {
    println!("Function Ran");
}

fn another_function(x: i8, y: &str) {
    println!("{}", x);
    println!("{}", y);
}

fn five() -> i32 {
    // -> i8 == return type.
    5 // is actually returning 127 just like below
      // return 127; // i8 type does not allow integer greater than 127
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
