use std::{cell::RefCell, rc::Weak};
use std::ops::Deref;

// A cons BoxList from Lisp
// Recursive data type
// The size of this kind of type cannot be determined
// during compile time
// It is like Node inside of a Linked BoxList
// enum BoxList {
//     Cons(i32, BoxList),
//     Nil,
// }
enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

// Custom Box
// Doesn't behave like the real Box
// Just to showcase implementing smart pointers
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Tuple's 0 is first value
        &self.0
    }
}

// To demonstrate drop trait
// What happens when things go out of bounds
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Demonstrate multi-ownership object
use std::rc::Rc;

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// Demonstrate combination of multi-ownership object
// with internal mutability during runtime
#[derive(Debug)]
enum RefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellList>),
    Nil,
}

// To demonstrate Reference Cycle
#[derive(Debug)]
enum CycleList {
    // I can understand why RefCell is needed, but why Rc is needed?
    // Oh, because it is owned by 2 instance, self and as other's next
    Cons(i32, RefCell<Rc<CycleList>>),
    Nil,
}

use crate::CycleList::{Cons as CycleCons, Nil as CycleNil};

impl CycleList {
    // Return the second item if the next Cons exist
    fn tail(&self) -> Option<&RefCell<Rc<CycleList>>> {
        match self {
            CycleCons(_, item) => Some(item),
            CycleNil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    // If it is not weak, you will cause a reference cycle
    // because both child and parent will own each other
    // A weak reference doesn't increment strong reference count
    // When strong reference count is 0, it is freed
    // In this case, the weak reference will not cause a memory leak
    // because of reference cycle since it doesn't own the target
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // Box smart pointer put the data on heap
    // Commonly used when:
    // 1. Unknown size during compile time, but requiring exact size
    // 2. Large data transfer ownership without copying data
    // 3. Own a value that implements a particular trait, not about the type
    // No overhead btw
    let b = Box::new(5);
    println!("b = {}", b);

    // You can import internal type from an enum
    use crate::BoxList::{Cons, Nil};
    // Use Box pointers to initialize
    let BoxList = Cons(1, 
        Box::new(Cons(2,
        Box::new(Cons(3,
        Box::new(Nil)
    )))));

    // Demonstrating ref and deref
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // Of course, this doesn't work
    // assert_eq!(5, y);

    // A box implements Deref trait
    // can be treated as a ref
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // My own Box type
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // *(y.deref()) is ran
    // This is how implicit deref coercions works
    // E.g. passing &Box<String> into &str parameter
    // String also implements deref
    // So you can pass String into &str
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // How it would look like without implicit deref
    // coercions
    // Deref box into String, take string slice,
    // ref to the slice
    hello(&(*m)[..]);
    
    // For more information, when there's mut:
    // Both uses T: Deref<Target=U>
    // &T -> &U
    // &mut T -> &U
    // Uses T: DerefMut<Target=U>
    // &mut T -> &mut U

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // Drop cannot be use explicitly
    // This is a destructor
    // a function that cleans up an instance
    // Will cause double free error.
    // c.drop();
    // In this case, simply call drop will do
    // std::mem::drop, included by default
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // Demonstrating Ref count Smart pointer
    // For multiple ownership
    // Think of connected List, Graph
    // Instead of managing lifetime reference, this is much easier
    // and also in some cases it is a must to have multiple ownership
    // to solve this kind of problems.

    // Note that Rc<T> allow multiple mutable references
    // How to deal with data races and inconsistencies in this case?
    use crate::RcList::{Cons as RcCons, Nil as RcNil};
    
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // The clones here only increments reference count
    // You can also use a.clone(), but it is 
    // more conventional this way

    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Combination of Rc and RefCell
    use crate::RefCellList::{Cons as RefCellCons, Nil as RefCellNil};

    // This is getting pretty confusing...
    // A multiowner, RefCell i32
    // Just the i32, remember
    let value = Rc::new(RefCell::new(5));

    // A multiowner, Cons that owns value
    let a = Rc::new(RefCellCons(Rc::clone(&value), Rc::new(RefCellNil)));

    // Two Cons that has value of 3, 4 and owns 'a'
    let b = RefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Mutate the immutable reference with RefCell
    // Remember, implicit deref multiple times here
    *value.borrow_mut() += 10;

    // Works as expected, a's value is 15
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Useful but... my god is it confusing
    // Sometimes it is more convenient to use this tho,
    // just a small trade off for speed (More overhead for counting)
    // RefCell<T> does not work with multithreaded code
    // That will be... Mutex<T>

    // Create a reference cycle
    // a = 5, next = Nil
    // let a = Rc::new(CycleCons(5, RefCell::new(Rc::new(CycleNil))));
 
    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    // // b = 10, next = a
    // let b = Rc::new(CycleCons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     // Set a's next to b
    //     // RefCell allows mutable
    //     // Now, this is what it looks like
    //     // CycleCons(5) -> CycleCons(10) -> CycleCons(5)
    //     //            b -^             a -^
    //     // CycleCons(5) and CycleCons(10) is RefCell pointing to the same
    //     // target a and b (Both are Rc) are pointing to
    //     // So, even if a and b is dropped, the memory cannot be freed
    //     // Basically, there are 4 pointers at this point, and 2 of them
    //     // points to each other as respective's next
    //     // meaning they owned each other
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // // Uncomment the next line to see that we have a cycle;
    // // it will overflow the stack
    // // println!("a next item = {:?}", a.tail());

    // // After the main function ends, a and b's memory won't be freed
    // // because the reference count is still = 1 for both a and b
    // // after dropping, the Rc is still not freed because they owned each
    // // other

    // Demonstrating Weak Reference Count smart pointer
    // to avoid reference cycle
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        // dropping branch will not cause memory leak 
        // since strong count is 0 (only itself)
        // no reference cycle occurs
    }

    // Parent no longer exists
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // Only 1 strong, which is itself
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}