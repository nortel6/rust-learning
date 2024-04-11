// Lifetime in a struct
// Able to make sure the reference must be alive
// Cannot outlive the part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 3rd lifetime elision rule applies here
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("long string is long");
    // let string1 = String::from("xyz");
    // let result;
    {
        // let string2 = String::from("long string is long");
        let string2 = String::from("xyz");

        // With lifetime, compiler will be able to tell
        // whether it lives long enough to be use by result
        // In this case, it will know the reference of result
        // must be used before string2 is dropped.
        // result = longest(string1.as_str(), string2.as_str());
        let result = longest(string1.as_str(), string2.as_str());
        // result = if string1.as_str() > string2.as_str() {
        //     string1.as_str()
        // } else {
        //     string2.as_str()
        // }
        println!("The longest string is {}", result);
    }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // This lives through the entire program
    // static lifetime
    // All string literals have static lifetime
    let s: &'static str = "I have a static lifetime.";
}

// The borrow checker couldn't tell the lifetime of a reference
// in a function if you don't specify it like this
// because it wouldn't know which reference will be returned
// Since it contains a control flow statement
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// In this case, lifetime is implicitly added by compiler
// There's a feature called "lifetime elision rules"
// 3 rules:
// 1. assigns a lifetime parameter to each parameter that’s a reference
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
// 2. if there is exactly one input lifetime parameter, that lifetime 
//    is assigned to all output lifetime parameters
// fn foo<'a>(x: &'a i32) -> &'a i32
// 3. if there are multiple input lifetime parameters, but one of them 
//    is &self or &mut self because this is a method, the lifetime of
//    self is assigned to all output lifetime parameters. 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

use std::fmt::Display;

// With Generics, mix match, this is so damn weird lol
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}