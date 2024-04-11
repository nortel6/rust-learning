fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Type must be annotated
    // You can see the value in IDE
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadow previous declaration
    // This is useful for staying immutable after a transformation
    // Or when you want to reuse the same name, depends on the context
    /*
     * For example, refer to the following case:
     */
    // let spaces = "   ";
    // let spaces = spaces.len();
    //
    // This will generate compile error
    // let mut spaces = "   ";
    // spaces = spaces.len();
    let x = 5;

    // Doesn't have to be mutable to shadow previous declaration
    let x = x + 1;

    {
        // Innershadow does not go outside the scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Same as 98222, just visual separation
    // Integer literals
    let _y = 98_222;
    let _y = 0xff;
    let _y = 0o77;
    let _y = 0b1111_0000;
    let _y = b'A';

    // This will provide error for debug build, but not release
    // In release, the value will be wrapped around
    // 256 -> 0, 257 -> 1
    let mut _y: u8 = 255;
    // For some reason this is not true? I cannot compile it even with
    // --release
    // I guess it had to be some sort of unpredictable input
    // E.g. user input
    // _y += 1;
    // Can use wrapping method to avoid overflow
    // This is unstable feature tho, cannot be used normally.
    // _y = wrapping_add(_y, 1);

    // Default is f64
    let x = 2.0;
    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    // Unicode
    // Note that String uses double quote
    let heart_eyed_cat = '😻';

    // Tuple
    // Fixed length
    let tup = (500, 6.4, 1);
    // Destructure
    let (x, y, z) = tup;
    // Empty value
    let empty = ();

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // Array
    // Also fixed length
    // Must be same type
    let a = [1, 2, 3, 4, 5];
    // [3, 3, 3, 3, 3]
    let a = [3; 5];
    // Accessing
    // note that runtime index out of bound cannot be checked
    // Duh, because runtime cannot be predicted
    let first = a[0];
    // This can be found by compiler
    // let second = a[19];
    let second = a[1];
}