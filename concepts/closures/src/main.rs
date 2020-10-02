use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let intensity: u32 = rand::thread_rng().gen_range(1, 100);
    let random_number: u32 = rand::thread_rng().gen_range(1, 3);
    generate_workout(intensity, random_number);
    let num1 = 4;
    // closures read variables in scope
    let equal_to_num1 = |z| z == num1;
    let num2 = 4;
    assert!(
        equal_to_num1(num2),
        "num2 does not match num1.... num1: {}",
        num2
    );
}

#[allow(unused)]
fn simulated_expensive_calculation(x: u32) -> u32 {
    println!("This may take a moment...");
    thread::sleep(Duration::from_secs(2));
    x
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(mut v) => {
                v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[allow(dead_code, unused)]
fn generate_workout(x: u32, random_number: u32) {
    let _copy_number = |x: u32| x;
    let expensive_closure = |num: u32| -> u32 {
        println!("This may take a moment...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let this_looks_like_a_function_because_it_acts_like_one = |x: u32| -> u32 { x + 2 };
    println!(
        "This number is 2+2 = {}",
        this_looks_like_a_function_because_it_acts_like_one(2)
    );
    let mut expensive_result = Cacher::new(|num| {
        println!("This may take a moment...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if x < 25 {
        println!("Today, do {} pushups!", expensive_result.value(x));
        println!("Next, do {} situps!", expensive_result.value(x));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(x));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cacher;
    #[test]
    fn test_cacher() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(1, v1);
        assert_eq!(2, v2);
    }
}
