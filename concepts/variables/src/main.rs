const MAX_POINTS: u32 = 100_000;
fn main() {
    let sep = "===================";
    println!("{}", sep);
    let x = 10;
    println!("X: {}", x);
    println!("MAX: {}", MAX_POINTS);
    let x = 100; // immutable, this is called shadowing, essentially creating a new variable
    println!("X: {}", x);
    // let MAX_POINTS = 100_100; WONT WORK BECAUSE ITS A CONSTANT, IMMUTABLE FOREVER
    let mut y = 20;
    println!("Y: {}", y);
    y = 200; // mutable by default, can be reassigned
    println!("Y: {}", y);
    println!("{}", sep);
    /*
        Mutable != YOU CAN CHANGE THE TYPEOF VARIABLE
        EXAMPLE BELOW:
    */
    println!("{}", sep);
    println!("Mutable != Changeable Type Declaration");
    // let mut spaces = "   ";
    // spaces = spaces.len(); This will give error as typeof mut spaces == string, then attempted to be redeclared as an int
    let spaces = "   ";
    println!(
        "This is the string of variable spaces: {}, notice the gap",
        spaces
    );
    let spaces = spaces.len(); // new variable shadowing the string type of spaces, NEW variable.
    println!("{}", spaces);
    println!("{}", sep);
}
