use getaver;
// Rust里面没有继承的概念，可以通过trait来进行行为的共享
// trait A {
//     fn sum() {
//         todo
//     }
// }

// struct T {
// }

// impl A for T {
//    fn sum() {}
// }

fn main() {
    let mut a = getaver::AverCollect::new();
    a.add(1);
    println!("average = {}", a.average());

    a.add(2);
    println!("average = {}", a.average());

    a.add(3);
    println!("average = {}", a.average());

    a.remove();
    println!("average = {}", a.average());

    println!("Hello, world!");
}
