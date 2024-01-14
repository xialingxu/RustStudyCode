pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    // LimitTracker结构体有一个字段messenger，它是一个实现了Messenger trait的类型的引用。
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    // 为了使用send方法，我们需要在T上实现Messenger trait。
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        // LimitTracker结构体有一个字段messenger，它是一个实现了Messenger trait的类型的引用。
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        // 为了使用send方法，我们需要在T上实现Messenger trait。
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        // 为了使用send方法，我们需要在T上实现Messenger trait。
        if percentage_of_max >= 1.0 {
            // 为了使用send方法，我们需要在T上实现Messenger trait。
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            // 为了使用send方法，我们需要在T上实现Messenger trait。
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            // 为了使用send方法，我们需要在T上实现Messenger trait。
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // MockMessenger结构体有一个叫sent_messages的字段，它是一个可变的vector。
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        // MockMessenger结构体有一个叫sent_messages的字段，它是一个可变的vector。
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // MockMessenger结构体有一个叫sent_messages的字段，它是一个可变的vector。
        fn send(&self, message: &str) {
            // MockMessenger结构体有一个叫sent_messages的字段，它是一个可变的vector。
            self.sent_messages.borrow_mut().push(String::from(message));
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();  
            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // MockMessenger结构体有一个叫sent_messages的字段，它是一个可变的vector。
        let mock_messenger = MockMessenger::new();
        // MockMessenger结构体有一个叫sent_messages的字段，它是一个可变的vector。
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // MockMessenger结构体有一个叫sent_messages的字段，它是一个可变的vector。
        limit_tracker.set_value(80);

        // MockMessenger结构体有一个叫sent_messages的字段，它是一个可变的vector。
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}