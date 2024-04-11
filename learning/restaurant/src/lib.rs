// Move definition to another file
mod front_of_house; //{
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// Bring the hosting scope to here
// use crate::front_of_house::hosting;
// Reexporting the module, it will be available to call
// with short form for anyone that uses this module
// restaurant::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting;
// Can further shorten it
// Although, this is too short
// Makes it unclear where this function comes from when
// calling it
// This is only idiomatic if there's multiple same named functions
// In this case, add "as <new_name>" to differentiate the call
use crate::front_of_house::hosting::add_to_waitlist;

// Note that this won't work, because it doesn't belong to the
// same scope as where the "use" keyword is invoked
// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

// You can also bring multiple scope in one line
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// If self import as well
// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// Or... import everything
use std::collections::*;

// front_of_house doesn't need pub
// because this function is in the same
// module
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // Shortened with the "use" keyword
    hosting::add_to_waitlist();
    add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Go out by 1 module
        // Doesn't need pub btw
        // child can access parent's
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        // Need pub for the fields
        // for eat_at_restaurant
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Enum doesn't need pub for fields
    // Make sense if you think about it
    // Since enum is a custom-defined type (field)
    // While struct is a group of types (fields)
    pub enum Appetizer {
        Soup,
        Salad,
    }
}