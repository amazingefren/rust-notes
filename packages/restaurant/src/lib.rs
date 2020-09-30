// #[cfg(test)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            seat_at_table() // pub mod fn can send request to private
        }
        fn seat_at_table() {
            println!("test");
        }
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// fn main() {
//     eat_at_restaurant();
// }
//
use self::front_of_house::hosting as something_else; // last thing on this import will be created as a variable

pub fn eat_at_restaurant() {
    //Absolute Path
    // crate::front_of_house::hosting::add_to_waitlist();

    //Relative Path
    // front_of_house::hosting::add_to_waitlist(); // can request public fn not private
    //
    something_else::add_to_waitlist();
    something_else::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("The season fruit is {}", meal.seasonal_fruits);
    let order1 = second_of_house::Appetizer::Soup;
    let order2 = second_of_house::Appetizer::Salad;
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruits: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruits: String::from("Peaches"),
            };
        }
    }
}

mod second_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
