// 1. 通过Send允许在线程间转移所有权
// 2. Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用

fn main() {
    println!("Hello, world!");
}
