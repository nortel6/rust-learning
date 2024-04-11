// In my understanding, trait is similar to that of interface in golang
// Well, it just looks like interface for OOP as well

// You can implement any traits for any type defined
// locally, within aggregator
// You can also implement local traits for any type
// But not for external types with external traits
pub trait Summary {
    // If no implementation by default
    // fn summarize(&self) -> String;
    
    // If default implementation exists
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Accepts type that implement Summary...
// What the hell
// I mean, if you think of it as Interface
// it makes sense

pub fn notify(item: &impl Summary) {
// Equivalent to the signature
// This is also called Trait Bounds
// If you implement a trait, you are the trait
// Think of trait as a type
// pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// in the case of multi parameters, this form assumes 
// different types
// pub fn notify(item1: &impl Summary, item2: &impl Summary)

// In this case, this form can help enforce same type,
// not just implementing the same trait
// pub fn notify<T: Summary>(item1: &T, item2: &T)

// Of course you have this kind of thing as well...
// Implement multiple traits
// pub fn notify(item: &(impl Summary + Display))

// Generic form equivalent, this is weird but understandable
// pub fn notify<T: Summary + Display>(item: &T)

// If you have a complex type for the traits...
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32

// You can use 'where' clause to define the type in a more readable way
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,

// Returning a type that implements trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// However, you cannot return multiple types that implement
// the same trait, at least not atm in Chapter 10
// Wait until Chapter 17
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// For all the Pair<T> that implements the Display + PartialOrd trait
// Conditionally implements methods depending on traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// You can also conditionally implement trait
// depending on the trait, lol, crazy
// impl<T: Display> ToString for T {
//     // --snip--
// }