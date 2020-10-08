use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

#[allow(unused, dead_code)]
fn mspc_main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("Hello World!");
        tx1.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("I'm"),
            String::from("Efren"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
#[allow(unused, dead_code)]
fn thread_main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi Number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    let handle2 = thread::spawn(move || {
        for _i in 1..5 {
            println!("Here's vector {:?}", v);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();
    handle2.join().unwrap();
    // handle.thread();
    for i in 1..30 {
        println!("Hi Number {} from main thread", i);
        thread::sleep(Duration::from_millis(100));
    }
}

#[allow(dead_code, unused)]
fn old_main() {
    println!("Hello, world!");
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi Number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi Number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}
