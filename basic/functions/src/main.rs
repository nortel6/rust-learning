fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // Statement and Expression
    // Rust is expression-based language
    // This is a statement
    // let y = 6;
    // Statement does not return value
    // This will error
    // let x = (let y = 6);

    // The expression evaluates to 4
    // valid in Rust
    let y = {
        let x = 3;
        // No semicolon for expression
        // If statement, add semicolon
        x + 1
    };

    println!("The value of y is: {y}");

    let y = five();
    println!("Five: {y}");
}

// I just realized
// Type for the parameter looks like go syntax
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// Multi parameter
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Return 5, implicit return expression
fn five() -> i32 {
    // Putting semicolon ends up as statement
    // = Error
    5
}
