/*
    Box<T> stores data on the heap by default
        - Don't have performance overhead, don't provide many capabilities either.
        - Use cases:
            * When size of value can't be known at compile time
            * Large amount of data and want to transfer ownership to prevent duplicating/copying data
*/
#[derive(Debug)]
enum Lists {
    Cons(Rc<RefCell<i32>>, Rc<Lists>),
    Nil,
}

use crate::Lists::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// RC<T>  is like a living room, theres a tv and people watching it, when the first person comes in, its on. when the last person leaves, it turns off.
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// use crate::List::{Cons, Nil};

#[allow(unused)]
fn _rc_main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a)); // a is moved into b, b owns a.
    // let c = Cons(4, Box::new(a)); // by now, a has been moved
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("Count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("Count after creating b = {}", Rc::strong_count(&a));
    // {
    // let c = Cons(4, Rc::clone(&a));
    // println!("Count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("Count after C goes out of scope = {}", Rc::strong_count(&a));
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
// enum List {
// Cons(i32, Box<List>), // providing box here, prevents List from being infinitely scalable
// Nil,
// }

// use crate::List::{Cons, Nil};

#[allow(unused)]
fn _old_main() {
    let b = Box::new(5); //i32 is stored on heap
    println!("b = {}", b);
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
