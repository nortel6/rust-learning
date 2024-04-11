enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Using struct would make this very painful
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// You can implement enum as well
// This is like superclass
// Polymorphism?
impl Message {
    fn call(&self) {

    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // Compared to struct, Enum is more convenient in representing
    // similar objects in different form.
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Options
    // Very useful for handling "null" values
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Others
        // By default, the last pattern will match all values not listed
        // This works, but not nice
        // others => reroll(),
        // This is better
        _ => reroll(),
        // If nothing need to be done, use unit value
        // _ => ()
    }

    let config_max = Some(3u8);
    // Normal matching
    // Can get annoying when we only want to match 1
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    // using if let instead, specifically match 1
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    // You can also use else, but this is basically _ => count += 1;
    } else {
        count += 1;
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    // Different flow for different values of Enum
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // You can also get the type as parameter...?
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // Validating Null
    match x {
        // All patterns must be covered
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}