use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    first_channel_example();
    multiple_values_example();
    multiple_producer_example();
}

fn first_channel_example() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hello!");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got {}", received);
}

fn multiple_values_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn multiple_producer_example() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let values = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let values = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}