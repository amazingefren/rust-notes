/*
    match VALUE {
        PATTERN=>EXPRESSION,
        PATTERN=>EXPRESSION,
        PATTERN=>EXPRESSION,
        PATTERN=>EXPRESSION,
    }
*/
fn main() {
    println!("_______ if_let ________");
    if_let();
    println!("_______ while_let ________");
    while_let();
    println!("_______ for_loop ________");
    for_loop();
    println!("_______ let_state ________");
    let_state();
    println!("_______ fnc_param ________");
    let cords = (-512, 1024);
    fnc_param(&cords);
    println!("_______ match_option ________");
    match_option();
    println!("_______ use_point ________");
    destruct_struct();
    println!("_______ destruct_enum ________");
    destruct_enum();
    println!("_______ ignore_value ________");
    ignore_value(5, 10);
    println!("_______ at_bindings ________");
    at_bindings();
}

fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("USing your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is Green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("TOP = {0}", top)
    }
}

fn for_loop() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at the index {}", value, index);
    }
}

fn let_state() {
    let a = 1;
    let (b, c, d) = (2, 3, 4);
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}

fn fnc_param(&(x, y): &(i32, i32)) {
    println!("Current Location ({}, {})", x, y);
}

fn match_option() {
    let x = 11;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("int: {}", x),
    }
    let y = Some(5);
    let z = 10;
    match y {
        Some(50) => println!("Got 50"),
        Some(z) => println!("Matched, z = {:?}", z),
        _ => println!("Default case, y = {:?}", y),
    }
    drop(x);
    // OR MATCHING
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("THREE"),
        _ => println!("Unknown::: int = {:?}", x),
    }
    println!("at the end: y = {:?}, z = {:?}", y, z);
    // Range Matching
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'x';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("Something Else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destruct_struct() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    println!("0 == {}", a);
    assert_eq!(7, b);
    println!("7 == {}", b);
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis ({}, {})", x, y),
    }
}

#[allow(unused)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destruct_enum() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    // let msg = Message::Write(String::from("Hello World!"));
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure"),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text Message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("R: {}, B: {}, B: {}", r, g, b),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("H: {}, S: {}, V: {}", h, s, v),
    }
}

#[allow(unused)]
fn ignore_value(_: i32, y: i32) {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    println!("Here's your y back :) ::: {}", y);
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some Numbers: {}, {}, {}", first, third, fifth);
        }
    }
    let _ignore_unused = "Hello World";
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    match numbers {
        (first, .., last) => println!("First: {}, Last: {}", first, last),
    }
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    let x = Some(5);
    let y = 5;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 1;
    let y = true;
    match x {
        1 | 2 | 3 if y => println!("yes"),
        _ => println!("no"),
    }
}

#[allow(unused)]
fn at_bindings() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range : {}", id_variable),
        Message::Hello {
            id: id_variable @ 10..=12,
        } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
