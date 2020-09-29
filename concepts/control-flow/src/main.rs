/*
    'if' expressions:
        if 'condition' {statement}
            --extra: if 'bool' {statement}
        else {statement}



*/
fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    // Temp with floats
    let faren_temp: f64 = 65.12;
    let cels_temp: f64 = (faren_temp - 32.00) * (5 as f64 / 9 as f64);
    println!("F: {} = C: {}", faren_temp, cels_temp);

    // Fibonacci nth Number
    let ninth_fib = fib(10);
    println!("Ninth Fib: {}", ninth_fib);
}

fn _notes_main() {
    // with int condition
    let if_number = 0;
    if if_number < 5 {
        println!("if_number is less than five");
    } else {
        println!("if_number is greater than or equal to five");
    }
    // if if_number {statement} will not work, unlike js -- because it exists, does not make it true
    if if_number != 0 {
        println!("if_number is not 0");
    } else if if_number == 0 {
        println!("if_number is equal to 0");
    } else {
        println!("this would probably be an error, should never run");
    }

    // with boolean
    let true_condition = true;
    if true_condition {
        println!("if_true is {}", true_condition);
    }

    // using if in let statement
    let false_condition = false;
    let false_number = if false_condition { 5 } else { 6 }; // type delcaration is important here

    // let true_number = if true_condition {5} else {"Not True"} will not work as it expects i32 declared by section {5}
    println!("true_number: {}", false_number);

    // loop
    let mut count = 0;
    loop {
        println!("count: {}", count);
        if count == 2 {
            break count = 0; //reset count to 0
                             // break;
        }
        count += 1;
    }
    println!("count reset: {}", count);
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result of count is {}", result);
    // while
    let mut while_number = 3;
    while while_number <= 5 {
        println!("while_number: {}", while_number);
        while_number += 1;
    }
    let mut while_bool = true;
    while while_bool {
        println!("while_bool ran successfully");
        while_bool = false;
    }
    let while_collection = [10, 20, 30, 40, 50];
    let mut while_index = 0;
    while while_index < while_collection.len() {
        println!(
            "while_collection[{}]: {}",
            while_index, while_collection[while_index]
        );
        while_index += 1;
    }
    // for
    let for_collection: [i32; 5] = [10, 20, 30, 40, 50];
    // safer and reduces chance of going above array length compared to while
    for element in for_collection.iter() {
        println!("the value of element: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
