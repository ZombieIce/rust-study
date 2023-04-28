// 动态大小类型和Sized trait

// &str有两个值： str的地址和长度, &str的大小在编译时就可以知道，长度是 2*usize
// 动态大小类型的黄金规则: 必须将动态大小类型的值置于某种指针之后 Box<str>\Rc<str>
// 另一个动态大小类型就是 trait 比如 &trait 或者 Box<trait>
// fn generic<T: Sized>(t: T) == fn generic<T>(t: T)
// 放宽限制 fn generic<T: ?Sized>(t: &T)
fn main() {
    let s1: &str = "hello";
    let s2: &str = " world";


    println!("Hello, world!");
}
