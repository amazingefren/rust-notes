/*
    Box<T> stores data on the heap by default
        - Don't have performance overhead, don't provide many capabilities either.
        - Use cases:
            * When size of value can't be known at compile time
            * Large amount of data and want to transfer ownership to prevent duplicating/copying data
*/

#[allow(unused)]
fn main() {
    println!("Hello World")
}

#[allow(unused)]
struct CustomSmartPointer {
    data: String,
}

// impl Drop for CustomSmartPointer {
// fn drop(&mut self) {
// println!("Dropping CustomSmartPointer with data `{}`!", self.data);
// }
// }

use std::mem::drop;
#[allow(unused)]
fn _drop_main() {
    let c = CustomSmartPointer {
        data: String::from("My Stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("Other Stuff"),
    };
    drop(c);
    println!("CustomSmartPointer created.");
}

struct MyBox<T>(T);

#[allow(unused)]
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn _deref_main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // y ptr points to x ptr, by using *
                       //  we are dereferencing the value, giving us the value of x
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // Here with Box<T> we are pointing to a COPIED version of x

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // Here with Box<T> we are pointing to a COPIED version of x
    let m = MyBox::new(String::from("Hello"));
    hello(&m);
    assert_eq!("Hello", *m);
}

#[allow(unused)]
fn hello(name: &str) {
    println!("Hello {}", name);
}
// bad enum
// enum List {
// Cons(i32, List),
// Nil,
// }

//good enum
enum List {
    Cons(i32, Box<List>), // providing box here, prevents List from being infinitely scalable
    Nil,
}

use crate::List::{Cons, Nil};

#[allow(unused)]
fn _old_main() {
    let b = Box::new(5); //i32 is stored on heap
    println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
