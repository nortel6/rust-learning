// Associated Type trait
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // The type is specified here
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Compared to generics, associated type can only be
// implemented for a single type
// With generics, things like Iterator<String> for Counter
// might be implemented
// So, you don't have to annotate the type
// for Counter explicitly with associated type


use std::{fmt, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Overloading Add operation for Point type
// By default, Add accepts and output self
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// This is the definition of Add trait
// <Rhs=Self> is a default type parameter
// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

struct Millimeters(u32);
struct Meters(u32);

// Able to change the default rhs is what makes
// this implementation possible
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Multiple trait with the same methods
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// To demonstrate explicit non-associated methods
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Super trait
// Trait bound for trait
// Will only works for type that implements
// Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // Use to_string() from Display
        // Thus, Display must be implemented first
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

// You are not able to implement Display for Vec
// since it is not defined in local traits
// However, you can bypass that using a wrapper
// also called the Newtype pattern
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let person = Human;
    // Explicitly specific the method of traits
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // When you don't have a self (non-associated)
    // you must tell Rust to use implementation of
    // Animal for Dog
    println!("A baby dog is called a {}", 
        <Dog as Animal>::baby_name());

    println!("{}", Point{x: 1, y: 3});
    Point{x: 1, y: 3 }.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}