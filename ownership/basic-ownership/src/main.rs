/*
    Ownership, Heap / Stack
    Heap, pointers to address
    Stack, Last in, First out faster

    - Each Value in Rust has a variable that's called its owner.
    - There can only be on owner at a time.
    - When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    scope();
    data();
    ownership();
    tuple_ownership();
}

fn scope() {
    //variable s does not exist here
    let _s = "Hello"; // immutable, takes only the space needed to store it
    let mut _input_s = String::new(); // mutable, can be used for user input, stored in heap allowing it to an unknown amount
    let mut string_s = String::from("Hello"); // mutable, default value, and still allowed to be changed to unknown amount in heap
    string_s.push_str(", World!");
    println!("{}", string_s)

    //variable s does exist here, do stuff with s
} //variable s stops existing, dropped automatically by rust

fn data() {
    let _x = 5; // x = 5
    let _y = _x; // y != x, y = 5 aka makes a copy
                 // strings are different.
    let _s1 = String::from("Hello"); // _s1 = "Hello"
                                     // on stack, s1 stores the ptr address, length, and capacity in bytes
                                     // on heap, [0]=H,[1]=e,[2]=l etc.
    let _s2 = _s1; /*
                   creates a 'stack' copy of _s1, NOT HEAP
                   This means that both values use the same data stored in heap, with two copies of their ptr,len,cap in the stack
                   Rust does this as creating copies in the heap would be expensive to the runtime performance
                   There is a issue here, Rust automatically drops the HEAP data after variable goes out of scope
                   This means that both s1, and s2 will try to drop the HEAP data (double free error)
                              println!("{}, world", _s1);
                   Wont Work^, Whats happening here is that when s2 = s1 is called, Rust considers s1 to be no longer valid
                   therefore rust doesn't have to call drop when s1 goes out of scope.
                   Summary: s1 is moved to s2 and s1 is no longer valid
                   s2 will clean the heap upon it going out of scope
                    */
    // Deep Cloning, Double Stack + Double Heap
    let _s1_deep = String::from("Hello");
    let _s2_deep = _s1_deep.clone();
    println!("s1: {}", _s1_deep);
    println!("s2: {}", _s2_deep);
}

fn ownership() {
    let s = String::from("Hello"); // s comes into scope
    takes_ownership(s); // s value moves into the function
                        // s is then no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into function, but i32 is a Copy
                   // x still valid here
    let s1 = String::from("World");
    let s2 = return_ownership(s1); // s1 is returned to new stack variable s2.
    println!("returned string: {}", s2); // therefore s1 now s2 is valid here
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn return_ownership(some_string: String) -> String {
    println!("Borrowed String: {}", some_string); // s1 aka some_string is valid here, can be used and altered etc.
    return some_string; // s1 is returned here
} // s1 is dropped.

fn tuple_ownership() {
    let s1 = String::from("Hello"); // s1 comes into scope
    let (s2, len) = calculate_length(s1); // s1 is returned here but the stack will be removed along with new variable length
    println!("the length of {} is: {}", s2, len); // s2 now exists and can be used here
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // s1 returned to be used with new variable, along with new variable
} // s1 dumped
