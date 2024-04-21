fn add_one(x: i32) -> i32 {
    x + 1
}

// In this case, it is fn, not Fn trait (closure)
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    // You can pass function pointer
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    
    // using the function instead of closure
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }

    // This is pretty cool way to initialize
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// To return closure, use trait object
// Also need dynamic since we don't know the type size at compile time
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}