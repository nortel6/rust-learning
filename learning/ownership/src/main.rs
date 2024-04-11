fn main() {

    let s1 = String::from("hello");
    // For objects with undeterminable size at compile-time
    // Assigning like this will invoke "drop" trait
    // Ownership will be transfer to s2 in this case.
    let s2 = s1;

    // s1 lost the ownership, no longer possible
    // println!("{}, world!", s1);


    let s1 = String::from("hello");
    let s2 = s1.clone();

    // This is ok, s2 cloned the heap memory data
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    // For data that's stack-only
    // It has "copy" trait
    println!("x = {}, y = {}", x, y);

    // s comes into scope
    let s = String::from("hello");  
    // s's value moves into the function...
    takes_ownership(s);
    // ... and so is no longer valid here
    // x comes into scope
    let x = 5;

    // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
    makes_copy(x);
    
    // gives_ownership moves its return
    // value into s1
    let s1 = gives_ownership();
    // s2 comes into scope
    let s2 = String::from("hello");
    // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
    let s3 = takes_and_gives_back(s2);
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    let len = calculate_length_with_reference(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Original must be mutable
    // if reference needs to be mutable
    let mut s = String::from("hello");

    change(&mut s);

    let mut s = String::from("hello");

    // let r1 = &mut s;
    // You cannot have multiple mutable reference at one time
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // You can do this though...
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;



    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // You cannot have a mutable reference
    // when there is read-only reference at the same time
    // RACE condition
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    // You can do this instead
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


    // Demonstrating slices
    // This is just like Golang...
    let my_string = String::from("hello world");

    // Slices = Partial Reference is my understanding
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);


    // Slices for array
    let a = [1, 2, 3, 4, 5];

    // Slice type is &[i32]
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {            
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours");
    // some_string comes into scope
    
    // some_string is returned and
    // moves out to the calling
    // function
    some_string
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { 
    // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    // len() returns the length of a String
    let length = s.len(); 
    (s, length)
}

// Much better with reference
fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

// Reference are immutable by default
// Compile error
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// This returns reference to nowhere
// Cannot be compile
// Also called dangling pointers
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// This is good
fn no_dangle() -> String {
    let s = String::from("hello");

    // Move ownership out, nothing to drop
    s
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Since there's no ability to return
            // partial part of a String
            // Return slices of reference instead
            return &s[0..i];
        }
    }

    &s[..]
}
