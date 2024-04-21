use std::slice;

// All extern is unsafe
// Using C's Application Binary Interface
extern "C" {
    fn abs(input: i32) -> i32;
}

// You can also create a function for other languages to call
// Do not mangle function's name while compiling
// This extern does not need to call unsafe
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Global variables
// Difference of static and constant:
// - Static is fixed memory address
// - Static can be mutable (This is unsafe)
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Unsafe trait and implementation
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    // Unsafe rusts superpowers showcase

    let mut num = 5;

    // Raw pointers
    // For raw pointers, immutable means
    // pointer cannot be directly assigned
    // to after being dereferenced.
    // Also, data races can happen with raw pointers
    // The usual rule of multi-read or single mut
    // doesn't apply here
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // We cannot be sure if raw pointers are valid
    // For example:
    // let address = 0x012345usize;
    // let r = address as *const i32;
    unsafe {
        // Creating raw pointers does no harm
        // But accessing the memory is unsafe
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Note that you don't have to
    // use unsafe block in a unsafe function
    unsafe fn dangerous() {}

    // Unsafe functions must be called within
    // unsafe block
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    // Demonstration of safe function
    // wrapping unsafe code
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


    add_to_count(3);

    unsafe {
        // Accessible mutable static is unsafe
        println!("COUNTER: {}", COUNTER);
    }
    // Also, union is unsafe, it is primary used to interact
    // with unions in C
}

// Example of safe implementation for split_at_mut
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    // This doesn't work because of multiple mut
    // borrow, borrow-checker doesn't know
    // that we are different parts, but same slice
    // (&mut values[..mid], &mut values[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
