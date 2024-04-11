// For printing value
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    // This won't work because of 'lifetime'
    // username: &str,
    // email: &str,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods, implement like Golang
impl Rectangle {
    // Associated function, associated with Rectangle struct
    // Or just call it Constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // &self is self: &Self
    // reference to self of type Self
    fn area(&self) -> u32 {
        // Also automatically reference/dereference self
        // Technically, (&rect).area() is called
        // But you can do rect.area() as well
        self.width * self.height
    }

    // Takes in another Rectangle ref
    // Can you use &Self? Yes you can
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// You can also have multiple impl
// Just like Golang
impl Rectangle {
    fn print(&self) {
        println!("{:?}", self);
    }
}


// Sometimes we want to give tuples a meaning
// Easier and make more sense to manage
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
// Similar to ()
struct AlwaysEqual;

fn main() {
    // Note that mut applies to all fields
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Getting the rest of the properties from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Prints to stdout
    println!("user2 is {:?}", user2);
    // Prints to stderr
    dbg!(user2);

    // Tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-like struct
    // Useful in traits? Apparently
    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // For associated function, use double colons
    let rect2 = Rectangle::square(5);
    rect2.print();
}

fn build_user(email: String, username: String) -> User {
    // Field init shorthand
    // Same name = Same field
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}