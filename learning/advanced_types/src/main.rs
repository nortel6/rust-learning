// Some types is dynamic sized type
// Meaning that it works like &str, which you have memory address
// and the length of the string

// For generic
fn generic<T>(t: T) {
    // --snip--
}

// By default, for the type which size can be determined during 
// compile time, <T: Sized> is implemented implicitly automatically
// Meaning the above function will looks like the following
// fn generic<T: Sized>(t: T) {
//     // --snip--
// }

// Although, you can relax the check using a ? Operator
// may or may not be Sized
// Only available for Sized trait
// Also since the type cannot be known compile-time, it must be &
// fn generic<T: ?Sized>(t: &T) {
//     // --snip--
// }

fn main() {
    // Alias
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Useful to shorten things to type
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    for n in 1..5 {
        let val = "123";
        let guess: u32 = match val.trim().parse() {
            Ok(num) => num,
            // Continue returns a !
            // which means no type
            // So compiler will set guess to u32
            Err(_) => continue,
        };
    }

    // There's a lot of use for the ! type
    // loop expression also returns ! so it loops forever
    // panic! also returns ! so it can return correct type when matched
}
