fn main() {
    // No init
    let v: Vec<i32> = Vec::new();
    // With Initialized values
    let v = vec![1, 2, 3];

    // Need mut, as usual
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // I am surprised I didn't have to comment this
    // I thought rust analyzer would know
    // let does_not_exist = &v[100];
    // Get returns a None if out of bounds, good
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // Cannot have mutable borrow
    // Pushing might reallocate memories
    // reference to it might be invalid later
    // v.push(6);
    println!("The first element is: {first}");

    // Looping through
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Can't use push/pop though
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    // enum vector, expected
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Droping vector drops its elements as well
    // Duh, obviously
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here




    // Strings
    // Remember literal = slies = &str
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    // Equivalent
    let s = String::from("initial contents");

    // Appending
    let mut s = String::from("foo");
    s.push_str("bar");

    // This works because &str doesn't take away ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push, only 1 character
    let mut s = String::from("lo");
    s.push('l');

    // Using + for
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 cannot be reference, it will be taken
    // Look into the "add" implementation for String
    // It takes in "self", not "&self"
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // Concat as well
    let s = format!("{s1}-{s2}-{s3}");

    // The reason why you cannot index a character in string directly
    // is because of how memory works for string
    // even slices wouldn't work, because some characters can be 2 bytes
    // Usually, we want the character, and not the bytes
    // But indexing a string could also mean returning the bytes instead
    // so indexing a string is treated as an error
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Woooo! Hashmap
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // Take out the value if it exists
    // If None, then 0
    // This is pretty complicated with ownership...
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // Ownership moves to hashmap
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!


    let mut scores = HashMap::new();

    // Overwritting value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Add if not exists
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Update based on old value
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
