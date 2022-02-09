use std::thread;
use std::time::Duration;

fn main() {
    let handle =thread::spawn(|| {
        for i in 1..10{
            println!("hi number {} from the spawned thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("number {} from main thread",i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Main Thread Finish Wait Other Thread");
    handle.join().unwrap();
}
