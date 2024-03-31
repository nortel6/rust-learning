fn main() {
    let mut counter = 0;

    // Loop is expression
    // Can also return a value...
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    // Loop label
    // kind of like GOTO
    // Must begin with a single quote
    let value = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // Exit inner loop
                break;
            }
            if count == 2 {
                // Exit counting_up loop
                break 'counting_up 10;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("End count = {count}, value = {value}");

    let mut number = 3;
    // While for conditional loop
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    // Loop through collections with for
    // Faster than while
    // Since it needs less checks
    // Less error-prone
    for element in a {
        println!("the value is: {element}");
    }

    // You can also do this...
    // rev = reverse
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}