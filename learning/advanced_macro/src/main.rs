use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// Implements HelloMacro trait
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}