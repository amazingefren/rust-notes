#![allow(unused)]
fn main() {
    println!("---------------------------");
    println!("number_list_bad -- Running");
    number_list_bad();
    println!("---------------------------");
    println!("number_list_good -- Running");
    number_list_good();
    println!("---------------------------");
    println!("long_char -- Running");
    long_char();
    println!("---------------------------");
    println!("generic_type -- Running");
    generic_type();
    println!("---------------------------");
    println!("struct_generic -- Running");
    struct_generic();
    println!("---------------------------");
}

// Struct Generic Type
/*
    Note: `You might be wondering whether there is a runtime cost when you’re using generic type parameters.
    The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.
    Rust accomplishes this by performing monomorphization of the code that is using generics at compile time.
    Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.`
*/
enum Option<T> {
    // Now Option makes alot more sense, as it is Option<T> Makes sense
    Some(T),
    None,
}
enum Result<T, E> {
    // So does Result, Ok(T) is a generic type, and so is Err(E) these values can be chars, ints, floats etc.
    // When running File::open
    Ok(T),  // We would assign std::fs::File here
    Err(E), // And this would be std::io::Error
}
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // Function X, takes (Self), has return type of Generic T, returns self.x being of type T
    // SO COOL
    fn x(&self) -> &T {
        &self.x
    }
}

// impl Point<f32> {
// fn distance_from_origin(&self) -> {
// (self.x.powi(2) + self.y.powi(2)).sqrt()
// }
// }

struct PointMix<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointMix<T, U> {
    fn mixup<V, W>(self, other: PointMix<V, W>) -> PointMix<T, W> {
        PointMix {
            x: self.x,
            y: other.y,
        }
    }
}

fn struct_generic() {
    // Mixing the generic type is NOT allowed on same Gen-Type
    let integer = Point { x: 5, y: 10 };
    let float = Point {
        x: 5.0,
        y: 10.00000,
    };
    let string = Point {
        x: "Hello",
        y: "John",
    };

    // PointMix has two generic types, T and U so mixing here is allowed
    let mix = PointMix { x: "Hello", y: 5.0 };
    println!("integer.x = {}", integer.x());
    let p1 = PointMix { x: 5, y: 10.4 };
    let p2 = PointMix {
        x: 10.04,
        y: "Hello",
    };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest_generic<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // for item in list { // CURRENTLY FOR THE COMPARISON TO WORK, std::cmp::PartialOrd must be used
    // if largest < item {
    // largest = item
    // }
    // }
    largest
}

fn generic_type() {
    let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    let number_list = vec![1, 25, 2500, 0, 30];
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for letter in list {
        if largest < letter {
            largest = letter
        }
    }
    largest
}

fn long_char() {
    let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    let result = largest_char(&char_list);
    println!("largest char: {}", result);
}

fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if largest < number {
            largest = number
        }
    }
    largest
}

fn number_list_good() {
    let vec1 = vec![34, 50, 25, 100, 65];
    let vec1_result = find_largest(&vec1);
    let vec2 = vec![102, 34, 6000, 89, 54, 2, 43, 9];
    let vec2_result = find_largest(&vec2);
    println!("vec1 largest result: {}", vec1_result);
    println!("vec2 largest result: {}", vec2_result);
}

fn number_list_bad() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    // giving ownership here intentionally
    for number in number_list {
        if largest < number {
            largest = number
        }
    }
    println!("Largest: {}", largest);

    // The reason this function is bad, is because to find the largest from a second array, requires duplicate code
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 9];
    let mut largest = number_list[0];
    for number in number_list {
        if largest < number {
            largest = number
        }
    }
    println!("Largest: {}", largest);
}
