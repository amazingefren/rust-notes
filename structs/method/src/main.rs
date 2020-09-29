#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Method
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("The Area of rect1 is {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect1));
    let square = Rectangle::square(10);
    println!("Area of square: {}", square.area())
}
