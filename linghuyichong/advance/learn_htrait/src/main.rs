use std::ops::Add;

// 1. 关联类型在 trait 定义中指定占位符类型

pub trait MyIterator<T> {
    fn next(&mut self) -> Option<T>;
}

#[derive(Debug)]
struct A {
    value: i32,
}

impl MyIterator<i32> for A {
    fn next(&mut self) -> Option<i32> {
        println!("in i32");
        if self.value > 3 {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.y,
            y: self.y + rhs.x,
        }
    }
}
#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

trait AA {
    fn print(&self);
}
trait BB {
    fn print(&self);
}
struct MyType;

impl AA for MyType {
    fn print(&self) {
        println!("AA trait for MyType");
    }
}

impl BB for MyType {
    fn print(&self) {
        println!("BB trait for MyType");
    }
}

impl MyType {
    fn print(&self) {
        println!("MyType");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;
impl Dog {
    fn baby_name() -> String {
        "Dog".to_string()
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        "Puppy".to_string()
    }
}

// 父 trait
use std::fmt;
trait OutPrint: fmt::Display {
    // 要求实现 Display trait
    fn out_print(&self) {
        let output = self.to_string();
        println!("output: {}", output);
    }
}

impl OutPrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// newType 模式用以在外部类型上实现外部 trait
// 孤儿规则： 只要trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait
// 一个绕开这个限制的方法是使用 newType 模式
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0.join(","))
    }
}

// 类型别名
type Kilometers = i32;
// 1. 考虑类型 Box<dyn Fn() + Send + 'static>
// type Trunk = Box<dyn Fn() + Send + 'static>


fn main() {
    let mut a = A { value: 3 };
    println!("next = {:?}", <A as MyIterator<i32>>::next(&mut a));
    println!("Hello, world!");

    let pa = Point { x: 1, y: 2 };
    let pb = Point { x: 3, y: 1 };
    println!("{:?}", pa + pb);

    let mi = Millimeters(1);
    let m = Meters(1);
    let r = mi + m;
    println!("r = {:?}", r);

    let my_type = MyType;
    my_type.print();
    MyType::print(&my_type);
    AA::print(&my_type);
    BB::print(&my_type);

    println!("baby_name: {}", Dog::baby_name());
    println!("baby_name: {}", <Dog as Animal>::baby_name()); // 完全限定语法
                                                             // <Type as Trait>::function(...)
    
    let pa = Point { x: 1, y: 2 };
    pa.out_print();


    let w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
    println!("w = {}", w);

    let x: i32 = 5;
    let y: Kilometers = 6;
    let r = x + y;
    println!("x + y = {}", r);

}
