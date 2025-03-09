mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Super will access the parent module, in this case, the root, crate
        super::deliver_order();
    }
    fn cook_order() {}

    // Example of public enum
    pub struct Breakfast{
        pub toast: String,
        fruit: String,
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("Peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_breakfast() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = Strign::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile since the fruit is not public
    // meal.fruit = String::from("Fruta");
}

pub fn eat_the_appetizer() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// This is only for this scope 
use crate::front_of_house::hosting;

// The module below will not compile because use is not within scope.
// mod customer {
    // pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist();
    // }
// }

// This will make the hosting available to any new scope.
// More details on page 300
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_with_use() {
    hosting::add_to_waitlist();
}

// Nested use
use std::{cmp::Ordering, io::Read};

// Nested with self
use std::io::{self, Write};


// glob operator, useful for tests, not so much for production
use std::collections::*;

// External modules
mod front_of_house_ext;

pub use crate::front_of_house_ext::hosting as hoster;

pub fn eat_at_restaurant_ext() {
    hoster::add_to_waitlist();
}