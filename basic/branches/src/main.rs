fn main() {
    let number = 3;

    // Just like Golang
    // {} block is called arms
    // Just like using 'match' keyword
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Unlike python, you cannot do this
    // strongly typed
    // if number {
    //     println!("Number is {number}");
    // }
    // Do this instead
    if number != 0 {
        println!("Number is not 0; number = {number}");
    }

    // Same old else if friend
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if to assign value
    // Since 'if' is an expression, it works
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // This won't work, must be same type for the expression
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}