use std::thread;
use std::time::Duration;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("number {} in spawn thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("number {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // println!("Hello, world!");
    // handle.join().unwrap();

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} in spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("number {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("v: {:?}", v);
    });

    // println!("v: {:?}", v); 移动后不能使用

    handle.join().unwrap();
}
