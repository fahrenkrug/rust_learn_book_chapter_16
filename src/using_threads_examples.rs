use std::thread;
use std::time::Duration;

pub fn run() {
    first_example();
    use_data_from_main_thread_example();
}

fn first_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn use_data_from_main_thread_example() {
    let v = vec![1,2,3];
    // Move tells the compiler that the closure should take the ownership of the used values.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); //Dropping v in the main thread while it may still be used in the thread ->
    // this is blocked by the compiler
    handle.join().unwrap();
}