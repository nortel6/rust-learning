use std::{fs::{self, File}, io::{self, ErrorKind, Read}};

fn main() {
    // This returns a Result
    // Since it can potentially fails
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        // You can also match multiple kind of errors
        // just like exceptions, beautiful
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Alternative using closures
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Unwrap will call panic if there's an error
    let greeting_file = File::open("hello.txt").unwrap();

    // Expect can provides a message, same as unwrap other than that
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");  

    // Short summaries
    /* 
     * panic is more suitable for testing and prototyping, as a placeholder
     * of some sorts.
     * or when you think it is logically impossible to fail
     * Example:
     * use std::net::IpAddr;
     *
     * let home: IpAddr = "127.0.0.1"
     *   .parse()
     *   .expect("Hardcoded IP address should be valid");
     * 
     * Use panics whenever an assumption is somehow broken
     * or... you can delegate how to handles an error back to the caller
     * if it make sense in the context.
     * 
     * For example, if the external code you are calling is out of your control
     * you should probably panic, because there might not be way to recover.
     * 
     * When an error is recoverable, try to always return Result instead
     * given that it make sense in the context to do so, of course
     * 
     * Although, sometimes panic is better for security reasons, if it is dangeorus
     * to move forward
     */
}

// Example of implementing an error check in a Type
// This doesn't work well for, say user-input tho
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// You can also return in main, but then... you will need this "trait thingy"
// it is like C's CLI apps I guess, returns 0 if success, and others if not
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

// This is basically how you 'throws' error back to calling functions
// Think of it as Java's throws in function signature
// Enum is really powerful... damn
fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // Shortcuts, WTF is ?
    // if Ok, then set as Ok's value
    // else early return Err(e)
    // The e here will follow the return type of Result, which is io::Error
    // in this case. What the?
    // This also means that ? operator can only be used when a functions
    // returns a Result
    // There are also other conditions, though
    // like Option, or another type that implements FromResidual
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // // You can further shorten it...
    // let mut username = String::new();

    // // You can chain it????
    // File::open("hello.txt")?.read_to_string(&mut username)?;

    // Ok(username)

    // Of course there's a built in...
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // The ? here means:
    // If None, returns None
    // Same logic follows
    text.lines().next()?.chars().last()
}