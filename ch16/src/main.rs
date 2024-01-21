use std::{
    sync::{
        mpsc::{self},
        Mutex,Arc,
    },
    thread::{self, spawn},
    time::Duration,
    rc::Rc,
};

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle1.join().unwrap();

    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    //drop(v); 所有权已经move到线程内部，这里不能再使用

    handle2.join().unwrap();

    /////////////////////////////////线程通信/////////////////////////////////////
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        //println!("val is {}", val); //所有权已经move到线程内部，这里不能再使用
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    /////////////////////////////////多个生产者/////////////////////////////////////
    println!("///////////////////////////////多个生产者////////////////////////////////////");
    let (tx1, rx1) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        //clone后，tx1和tx2都可以发送消息
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx1 {
        println!("Got: {}", received);
    }

    /////////////////////////////////共享状态的并发/////////////////////////////////////
    println!("///////////////////////////////共享状态的并发////////////////////////////////////");
    //counter的所有权被move到线程内部，因此无法在多个线程中共享；用Rc<T>也不行，因为Rc<T>不是线程安全的
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
