#![allow(dead_code)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
    VFake,
}

/*
    There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
    Version four type IP addresses will always have four numeric components that will have values between 0 and 255.
    If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct.
    Enums handle this case with ease:
*/

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// vs
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        //later
    }
}

fn main() {
    let _home = IpAddrKind::V4(127, 0, 0, 1); // cleaner gets rid of unneeded struct
    let _alt = IpAddr {
        kind: IpAddrKind::VFake,
        address: String::from("127.0.0.1"),
    };
    let _some_number = Some(5);
    // Note Option::Some(int) will not add with a normal i32 Int.
    // let wont_work = 5 + some_number;
    // This is because null is bad, rust makes you convert from Some/None to a value, this way you can ensure that there will not be a null value when needed
    let _some_string = Some("A String");
    let mut _absent_number: Option<i32> = None;

    let m = Message::Write(String::from("hello"));
    m.call();
}

// fn route(ip_kind: IpAddrKind) {}
