// used to demonstrate RefCell smart pointer
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // In order to test this, a mock object is needed
    // because set_value doesn't return anything
    // How do you know it sent the corret message?
    // The message is not printed in this case, nor is it
    // accessible
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A mock messenger object to see the messages
    // struct MockMessenger {
    //     sent_messages: Vec<String>,
    // }

    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger {
    //             sent_messages: vec![],
    //         }
    //     }
    // }

    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         // Since the send() method for the Messager trait
    //         // takes immutable reference, you cannot use it...
    //         // This is where 'interior mutability' helps
    //         self.sent_messages.push(String::from(message));
    //     }
    // }
    
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    use std::cell::RefCell;

    struct MockMessenger {
        // Is now a RefCell
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // RefCell<Vec<String>>
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Mutably borrows the value
            // Able to mutate the value even if it is immutable reference
            // This is the so called 'interior mutability'

            // Inside here, there are unsafe code (which could mean
            // something like mutating an immubtale reference),
            // compiler doesn't like this, but it doesn't mean
            // it is neccessarily unsafe, it just delegate
            // the rules to us to enforce

            // Compiler will not complain during compile time,
            // any violation of the borrowing rule during runtime
            // will panic instead

            // borrow_mut() is basically &mut
            // borrow() is &
            // RefCell<T> keeps track of count of Ref<T> and RefMut<T>
            // We can have many immutable, or 1 mutable borrow

            // In this case, we know that it is completely safe to do
            self.sent_messages.borrow_mut().push(String::from(message));
            
            // These 2 will panic at runtime
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();
        }
    }
}