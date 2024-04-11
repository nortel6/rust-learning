fn main() {
    // 0 1 1 2 3 5 8 13
    println!("n = 7, {}", fib(7));
    println!("n = 5, {}", fib(5));
    println!("n = 1, {}", fib(1));
    println!("n = 0, {}", fib(0));
}

fn fib(n: i32) -> i32 {
    let mut prev = 0;
    let mut curr = 1;
    let mut i = 1;
    // Expression magic...
    return if n < 1 { 0 } else { 
        loop {
            if i == n {
                break prev;
            } else { 
                let next = prev + curr;
                prev = curr;
                curr = next;
                i += 1;
            }
        }
    }
    // Apparently, you cannot do this
    // Looks like for is not an expression?
    // return for i in 1..n {
    //     if i == n {
    //         prev
    //     } else { 
    //         let next = prev + curr;
    //         prev = curr;
    //         curr = next;
    //         i += 1;
    //     }
    // };

    // But you can do this
    // for _ in 1..n {
    //     let next = prev + curr;
    //     prev = curr;
    //     curr = next;
    // }
    // prev
}