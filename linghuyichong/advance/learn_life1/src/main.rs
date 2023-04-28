// 1. Rust中每个引用都有其生命周期，也就是引用保持有效的作用域。大部分时候生命周期是隐含并可以推断的，正如大部分时候类型可以推断一样
// 2. 生命周期的主要目标是避免悬垂引用
// 3. Rust编译器使用借用检查器来检查生命周期是否有效

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// error
// fn a_str(x:&str, y: &str) -> &str {
// let r = String::from("abc");
// r.as_str()
// }
#[derive(Debug)]
struct A<'a> {
    name: &'a str
}

// 方法中的生命周期
struct StuA<'a> {
    name: &'a str,
}

impl <'a> StuA<'a> {
    fn do_something(&self) -> i32 {
        3
    }
    fn do_something2(&self, s: &str) -> &str {
        self.name
    }

    fn do_something3(&self, s: &'a str) -> &'a str {
        s
    }
    
}

// 静态生命周期
// 定义方式: 'static
// 其生命周期存活于整个程序期间，所有的字符字面值都拥有static生命周期

use std::fmt::Display;

fn function<'a, T:Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("ann is {}", ann);
    if x.len() < y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let s1 = "abcd";
    let s2 = "ab";
    let r = longest(s1, s2);
    println!("longest is {}", r);

    // let ss = a_str(s1, s2);
    // println!("ss = {}", ss);

    let n = "hello";
    let a = A{name: &n};
    println!("a = {:#?}", a);

    let a = StuA{name: "aaa"};
    println!("{}", a.do_something());
    println!("{}", a.do_something2("bbb"));
    println!("{}", a.do_something3("lslsls"));

    let s1 = String::from("i am s1");
    let s2 = String::from("i am s2, hello");
    let ann = 129;
    let r = function(&s1, &s2, ann);

}
