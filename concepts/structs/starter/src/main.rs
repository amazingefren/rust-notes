struct _User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct _Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The Area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    let rect2 = (30, 50);
    println!(
        "The Area of the rectangle is {} square pixels.",
        area_tuple(rect2)
    );
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The Area of the rectangle is {} square pixels.",
        area_struct(&rect3)
    );
    println!("Rect3 is {:?}", rect3);
}

fn area_struct(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn _old_main() {
    println!("Hello, world!");
    let mut user = _new_user(String::from("bob@live.com"), String::from("bob"));
    println!("Username: {}", user.username);
    user.username = String::from("test");
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("Sign in Count: {}", user.sign_in_count);
    println!("Active: {}", user.active);
    let _user2 = _User {
        email: String::from("fake@live.com"),
        username: String::from("john"),
        ..user
    };

    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("Sign in Count: {}", user.sign_in_count);
    println!("Active: {}", user.active);

    let _black = _Color(0, 0, 0);
}

fn _new_user(email: String, username: String) -> _User {
    _User {
        email, //shorthand, field and variable have the same name
        username,
        sign_in_count: 1,
        active: true,
    }
}
