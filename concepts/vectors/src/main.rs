#![allow(dead_code)]
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 30);
    scores.entry(String::from("Red")).or_insert(0);
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let vector_scores = HashMap<_, _> = teams.into.iter().zip(initial_scores.into_iter()).collect();
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    //Updating Value using original value
    let text = "Hello World Wonderful World";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

// Storing Strigs UTF-8
fn _string_main() {
    // let mut s1 = String::new();
    let data = "initial contents";
    let mut s1 = data.to_string();
    let s2 = "initial contents".to_string();
    println!("{}", s1);
    println!("{}", s2);
    s1 = "Hello World".to_string();
    println!("{}", s1);
    s1.push('!');
    println!("{}", s1);
    s1.push_str(" Hi From Efren");
    println!("{}", s1);
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let str1 = String::from("Hello");
    let mut str2 = String::from("שָׁלוֹם");
    let str3 = str1 + &str2;
    println!("{}", str3);
    // str1.push('h');
    str2.push('h');
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    println!("{}", _hello);
    let _hello1 = String::from("안녕하세요");
    let _hello2 = String::from("你好");
    let _hello3 = String::from("Olá");
    let hello_formatted = format!("{}, {}, {}", _hello1, _hello2, _hello3);
    println!("Ownership not taken ? {}", _hello1);
    println!("{}", hello_formatted);
    // let hello_russia = String::from("Здравствуйте");
    let hello_russia = "Здравствуйте";
    // let first_russian = &hello_russia[0]; will not work because utf-8 encoding means first character ranges past 1 byte
    let first_russian = &hello_russia[0..8];
    println!("First Russian: {}", first_russian);
    let hello_spanish = String::from("Hola");
    for c in hello_spanish.chars() {
        println!("{}", c);
    }
    for c in hello_spanish.bytes() {
        println!("{}", c);
    }
}
// Storing values with Vectors
fn _values_main() {
    let _v: Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3];
    v1.push(4);
    v1.push(5);
    let three: &i32 = &v1[2];
    println!("{}", three);

    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no element there"),
    }

    let mut v2 = vec![1, 2, 3, 4, 5];
    let first = &v2[0];
    println!("The first element is {}", first);
    v2.push(6);
    for number in &v2 {
        println!("{}", number);
    }
    v2.push(7);
    println!("--------------------");
    for number in &mut v2 {
        *number += 50;
    }
    for number in &v2 {
        println!("{}", number);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.001),
        SpreadsheetCell::Text(String::from("Hello World")),
    ];
    println!("{:?}", &row[2])
}
