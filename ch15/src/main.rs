use crate::List::{Cons, Nil};
use crate::ListRc::{Cons as otherCons, Nil as otherNil};
use crate::ListRcRefCell::{ConsRcRefCell, RcRefCellNil};
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = Box::new(x);
    assert_eq!(5, *z);

    let my = MyBox::new(x);
    assert_eq!(5, my.0);
    assert_eq!(5, *my);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //如果MyBox没有实现Deref的trait，那么需要hello(&(*m)[..])

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // let rc_a=Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let rc_b=Cons(3, Box::new(rc_a));
    // let rc_c=Cons(4, Box::new(rc_a));

    println!("------------------------------Rc<T>--------------------------------");
    let rc_a = Rc::new(otherCons(5, Rc::new(otherCons(10, Rc::new(otherNil)))));
    println!("count after creating a = {}", Rc::strong_count(&rc_a));
    let rc_b = otherCons(3, Rc::clone(&rc_a));
    println!("count after creating b = {}", Rc::strong_count(&rc_a));
    {
        let rc_c = otherCons(4, Rc::clone(&rc_a));
        println!("count after creating c = {}", Rc::strong_count(&rc_a));
    } //这里c出了作用域，但是因为rc_a和rc_b都还在作用域，所以他们的引用计数不会减少

    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&rc_a)
    );

    println!("-----------------------------RefCell<T>---------------------------------");
    // let v1=5;
    // let v2=&mut v1;
    let value=Rc::new(RefCell::new(5));

    let v1=Rc::new(ConsRcRefCell(Rc::clone(&value), Rc::new(RcRefCellNil)));
    let v2=ConsRcRefCell(Rc::new(RefCell::new(6)), Rc::clone(&v1));
    let v3=ConsRcRefCell(Rc::new(RefCell::new(10)), Rc::clone(&v1));

    *value.borrow_mut()+=10;

    println!("v1 is {:?}", v1);
    println!("v2 is {:?}", v2);
    println!("v3 is {:?}", v3);
    

}

#[derive(Debug)]
enum ListRcRefCell{
    ConsRcRefCell(Rc<RefCell<i32>>, Rc<ListRcRefCell>),
    RcRefCellNil,
}

enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

enum ListRc {
    // Cons(i32, List),
    Cons(i32, Rc<ListRc>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        //这里有点像finally 语句块
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
