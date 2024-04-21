struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}


fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // You can mix the let with more than just 'if'
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Loop as long as the match works
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    // destructuring tuple is a pattern 
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // This is also a pattern
    let (x, y, _) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);


    // Matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Surprisingly, it will match this instead
        // Because Named variables are irrefutable patterns
        // that match any values
        // Also, the y defined here is not the y outside
        // it will overshadow it
        // In this case, y will match any value
        // inside the Some() provided, this is Some(5)
        // from x
        // Thus, y = 5
        // In a sense, this is kind of destructuring
        Some(y) => println!("Matched, y = {y}"),
        // x must be None to match this
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 1;

    match x {
        // Multiple match
        // or operator
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        // Match ranges of value
        // Inclusive btw
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        // Cool
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    // Destructuring struct
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Shorthand
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // I think I am getting it now
    // destructuring is part of matching
    match p {
        // Any x value, y at 0
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        // still works for nested structs/enum
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // Complex destructuring
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // Can use _ to ignore values
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    let s = Some(String::from("Hello!"));

    // This is useful if you don't want to move s into the deconstruction
    // This will fail for example
    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        // Match x and ignores the remaining
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // Get the first and last only...
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
        // Note, you cannot be ambiguous when using ..
        // For example: (.., second, ..)
    }

    let num = Some(4);

    match num {
        // Deconstruct then... conditions
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Compared to before, this one actually works
        // like how you expect it
        // Compares variables to the external one after
        // deconstructing it into n
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;

    match x {
        // This means (4 | 5 | 6) if y => ...
        // Not 4 | 5 | (6 if y) => ...
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum MessageId {
        Hello { id: i32 },
    }

    let msg = MessageId::Hello { id: 5 };

    match msg {
        MessageId::Hello {
            // Long form deconstructuring of Struct inner value
            // for range
            // To bind it to a variable, use @
            // So weird...
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // Short forms are more intuitive, but you cannot 
        // bind it to a value
        MessageId::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageId::Hello { id } => println!("Found some other id: {}", id),
    }
}

// You can also do this
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// This is pretty weird
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}