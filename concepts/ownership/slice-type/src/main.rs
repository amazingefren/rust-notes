fn main() {
    programming_problem();
    intro_slice();
    programming_solved();
    array_slice();
}

fn programming_problem() {
    let s = String::from("What is going on");
    let word = first_word(&s);
    println!("first word result: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    // s.len()
    &s[..]
}

fn intro_slice() {
    let s = String::from("Hello World");
    let hello = &s[..5]; // .. == range syntax, in this case 0 can be dropped
    let world = &s[6..]; // dropping last essentially means to last byte
    let helloworld = &s[..]; // from 0 to last byte, essentially the whole thing
    println!("{} {}!", hello, world);
    println!("{}!", helloworld);
}

fn better_first(s: &str) -> &str {
    // recommended method as it can also take a slide of a string
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn programming_solved() {
    let s = String::from("What is going on");
    let word = better_first(&s[..8]);
    println!("better first word result: {}", word);
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];
    for item in slice {
        println!("{}", item);
    }
}
