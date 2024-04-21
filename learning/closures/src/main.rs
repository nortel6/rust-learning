use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    // These have to be called for closure to infer the type
    add_one_v3(1);
    add_one_v4(1);

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // The previous call causes the closure to be infer as String type
    // Thus, this one will face an error
    // let n = example_closure(5);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Base on how list is used
    // This closure will be inferred to only borrow
    // Since only immutable reference is needed
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Mutable borrows
    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Although the closure only require immubtale reference here
    // Using 'move' keyword can force it to take ownership of the list
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // This will be error
    // println!("After calling closure: {:?}", list);

    // When using closures, at least one of the 3 traits will be implemented:
    // All closures implement at least FnOnce
    // 1. FnOnce - Always, the only possible trait for
    //             closures that moves captured values out of its body
    // 2. FnMut - Mutate captured vaues
    // 3. Fn - Don't move captured values out, don't mutate captured values

    // You can look into the Option's "unwrap_or_else" method
}