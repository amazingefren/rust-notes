#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn second_test() {
        // panic!("This will fail the test")
        assert_eq!(1 + 1, 2);
    }
    use super::Rectangle;
    #[test]
    fn can_hold_test() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(larger._can_hold(&smaller));
    }
    #[test]
    fn double_can_hold() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(!smaller._can_hold(&larger));
    }
    use super::add_two;
    #[test]
    fn can_add_two() {
        assert_eq!(6, add_two(4)); // right is the to check. (x, y) x must equal y
    }
    #[test]
    fn do_not_equal() {
        assert_ne!(6, add_two(5)); // (x,y) x != y
    }
    use super::greeting;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Efren");
        assert!(
            result.contains("Efren"),
            "Greetings did not contain provided string, value was {}", // custom error code
            result // value for custom error code provided here
        ); // checks to see if type String, contains "Efren"
    }
    use super::Guess;
    #[test]
    #[should_panic] // makes TEST on FAIL be a success in this case, we are testing to ensure the function will fail
    fn greater_than_100() {
        Guess::new(200); // function will fail => test is a success
    }
    #[test]
    // A Result <T, E> test in this case <T == (), E> () being no return value
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
