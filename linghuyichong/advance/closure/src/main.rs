use std::{str, vec};

// 实现一个缓存，只处理第一次传入的值并保存
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let use_closure = || println!("This is a closure");
    use_closure();
    println!("Hello, world!");

    // 闭包定义会为每个参数和返回值类型推到一个具体的类型，但是不能推导两次
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    let b = add_one_v2(5);
    let c = add_one_v3(5);
    let d = add_one_v4(5);
    println!("b = {}, c = {}, d = {}", b, c, d);

    // 捕捉环境变量
    let i = 1;
    let exe = |x| x + i;
    let r = exe(5);
    println!("r = {}", r);
    

    // cacher
    let mut c = Cacher::new(|x| x+1);
    let v1 = c.value(1);
    println!("v1 = {}", v1);

    let v2 = c.value(2);
    println!("v2 = {}", v2);


// 闭包可以通过三种方式捕获其环境变量，获取所有权 可变借用 不可变借用
// FnOnce消费从周围作用域的变量。闭包获取其所有权并在定义闭包时将其移进闭包。其名称的Once部分代表了闭包不能多次获取相同变量的所有权。
// FnMut 获取可变的借用值，所以可以改变其环境
// Fn 从其环境获取不可变的借用值

    let x = 4;
    let equal_to_x = |z| z==x;
    let y = 4;
    assert!(equal_to_x(y));

}

// 语法格式
fn add_one_v1(x: u32) -> u32 {
    x + 1
}
