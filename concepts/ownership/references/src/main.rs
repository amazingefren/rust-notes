fn main() {
    reference_over_ownership();
    change_in_reference();
    mute_fail();
    mute_alt();
    _dangle_ref();
}

fn reference_over_ownership() {
    let s1 = String::from("hello"); // s1 initialized in stack+heap
    let len = calculate_length(&s1); // s1 is not transferred, address of heap is sent instead.
                                     // len is declared into scope by return of function
    println!("length of {} is: {}", s1, len); // s1 still in scope as it was never invalidated.
}

fn calculate_length(s: &String) -> usize {
    // s is defined as only a pointer, pointing to the pointer of s1
    // therefore heap memory of s1 can be read without creating a new item in the stack.
    s.len() // returning usize
} // s stack memory is dumped, heap of s1 stays

fn change_in_reference() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("s after change: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn mute_fail() {
    let mut s = String::from("Hello");
    {
        let _r2 = &mut s; // okay
    } // this works because r2 goes out of scope before r1 is referenced.
    let _r1 = &mut s; // okay
                      // let r2 = &mut s; // NOT OKAY THERE CAN ONLY BE ONE MUTATABLE REFERENCE
    {
        // let r2 = &mut s; // ALSO NOT OKAY BECAUSE MUTABLE REFERENCE r1 IS STILL IN SCOPE
    }
}

fn mute_alt() {
    let mut s = String::from("Hello");
    let r1 = &s; // okay!
    let r2 = &s; // okay!
    println!("{},{}", r1, r2); // okay, r1, r2 are unused after this point.
    let r3 = &mut s; // r3 is now a mutable reference and can be used, r1/r2 may no longer be used.
    println!("{} World!", r3); // okay!
}

fn _dangle_ref() {
    // let ref_to_nothing = dangle(); // will return a reference to a heap with nothing, compiler will stop you here
}

// fn dangle() -> &String {
// let s = String::from("Hello");
// &s // wont work because reference is to s, which will be dumped after it the local scope ends.
// }
