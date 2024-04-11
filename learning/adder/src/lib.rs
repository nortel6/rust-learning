// These are unit tests

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // format!("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }


        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// Private function is also testable
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// This macro tells that it shouldn't be included
// for the build
// Only test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    // You can also return a Result<T, E>
    // Cannot use should_panic macro
    // If you have a chain of operation that should
    // not return Err in the middle, don't use
    // question mark operator
    // To evaluate an Err, use assert!(value.is_err())
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // A test meant to fail
    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // Custom failure message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // #[should_panic]
    // To be more precise
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    // Pass in --ignored or --include-ignored to run this
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn internal() {
        // Testing private function
        assert_eq!(4, internal_adder(2, 2));
    }
}
