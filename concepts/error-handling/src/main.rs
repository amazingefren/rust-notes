#![allow(dead_code)]
#![allow(unused)]
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    //Unrecoverable Errors
    // panic!("Crash and Burn")
    // let v = vec![1, 2, 3];
    // v[99]; //index out of bounds panic
    let file_name = "hello";
    let f = File::open(&file_name); // return type Result<T, E>
                                    // let f = match f {
                                    //     Ok(file) => file,
                                    //     Err(error) => panic!("Problem Opening the File: {:?}", error),
                                    // };
                                    // use std::io::ErrorKind;
    let f = match f {
        Ok(file) => {
            println!("File Opened!");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    let f1 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
    //unwrap and expect are both handling the error catching for me, expect allows me to create a custom panic error
    // let f2 = File::open("hello.txt").unwrap();
    // let f3 = File::open("hello2.txt").unwrap(); // will throw panic for us if file fails
    // let f4 = File::open("hello2.txt").expect("This is a custom panic");
    // Propagatin Errors
    let f6 = read_username_from_file();
    let f7 = read_username_from_file_alt();
    // println!("{:?}", &f6);
    // println!("{:?}", &f7);
    let f8 = question_operator();
    println!("f8: {:?}", f8);
    possible_main_to_use_later();
}

// ? operator can be used in functions that return a Result<T, E>
fn question_operator() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // in this case if file is incorrect, it will panic! for us
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ? can be used in main function if \/
fn possible_main_to_use_later() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file_alt() -> Result<String, io::Error> {
    // println!("{:?}", std::fs::read_to_string("hello.txt"));
    std::fs::read_to_string("hello.txt")
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let f5 = File::open("hello5.txt");
    // let mut f5 = match f5 {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // match f5.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
