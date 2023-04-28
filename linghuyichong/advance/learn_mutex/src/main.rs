// 任意时刻，只允许一个线程来访问某些数据
// 互斥器使用时，需要先获到锁，使用后需要释放锁

// 1. Mutex<T> 提供内部可变性，类似于RefCell
// 2. RefCell<T>/Rc<T> 非线程安全的， Mutex<T>/Arc<T> 是线程安全的


use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // Mutex<T>是一个智能指针，lock调用返回一个叫 MutexGuard的智能指针
    // 内部提供了drop方法，实现当 MutexGuard离开作用域时自动释放锁

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // 离开作用域时，自动释放
    println!("m = {:?}", m);

    // Mutex实现的可变共享
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
            let cnt = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("result = {}", *counter.lock().unwrap());
}
