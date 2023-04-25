use std::fmt::Debug;

fn print_it<T: Debug + 'static> (input: T ) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it_1<T: Debug + ?Sized> (input: &'static T) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    let s = "hello"; // &str 等价于 &'static str
    print_it(s); // T: &str, 引用不是所有权类型，但是&str是'static生命周期的引用类型
    print_it_1(s); // 传入&str。T: str 具有'static 生命周期

    let i = 5;
    print_it(i); // i32是所有权类型
    print_it(&5); // 'static生命周期的引用类型

    print_it_1(&5); // 'static 生命周期的引用类型

    let s = String::from("hello");
    print_it(s); // 所有权类型
}