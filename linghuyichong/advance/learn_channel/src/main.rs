// 通过 mpsc::channel 创建通道，多个生产者，单个消费者
// 通过 spmc::channel 创建通道，多个消费者，单个生产者
// 创建通道后返回的是发送者和消费者
// let (tx, rx) = mpsc::channel();
// let (tx, rx) = spmc::channel();
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = tx.clone();

    thread::spawn(move || {
        let val = "hi".to_string();
        tx1.send(val).unwrap(); // send方法返回的是一个Result<T, E>，如果接收端已经被丢弃了，将没有发送值的目标，此时会返回错误
        // println!("val = {}", val); 调用send的时候，会发生move动作，所以此处不能再使用val

        thread::sleep(Duration::from_secs(2));

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let val = "hi".to_string();
        tx.send(val).unwrap(); // send方法返回的是一个Result<T, E>，如果接收端已经被丢弃了，将没有发送值的目标，此时会返回错误
        // println!("val = {}", val); 调用send的时候，会发生move动作，所以此处不能再使用val

        thread::sleep(Duration::from_secs(2));

        let vals = vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
            String::from("D"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    thread::spawn(move || {
        let val = "hi".to_string();
        tx2.send(val).unwrap(); // send方法返回的是一个Result<T, E>，如果接收端已经被丢弃了，将没有发送值的目标，此时会返回错误
        // println!("val = {}", val); 调用send的时候，会发生move动作，所以此处不能再使用val

        thread::sleep(Duration::from_secs(2));

        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // recv()返回值也是一个Result<T, E> 当通道发送端关闭时，返回一个错误值
    // recv()方法会阻塞到有一个消息到来。也可以使用try_recv()，不会阻塞，会立即返回
    for recv in rx {
        println!("Got: {}", recv);
    }

}
