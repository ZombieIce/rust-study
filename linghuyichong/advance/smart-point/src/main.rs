// 1. 指针是一个包含内存地址的变量。这个地址指向一些其他的数据
// 智能指针是一类数据结构，它们表现类似于指针，但是也拥有额外的元数据，最明显的，它们拥有一个引用计数。引用计数记录智能指针总共有多少个所有者

// 最简单最直接的智能指针的Box 将值放在堆上而不是栈上
// (1) 当有一个在编译时未知大小的类型，而又需要再确切大小的上下文中使用这个类型值的时候
// (2) 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的数据
// (3) 当希望拥有一个值并只关心它的类型是否实现了特定trait而不是其具体类型时

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 实现Deref trait 允许我们重载解引用运算符 let b = &a; let c = *b
use std::{cell::RefCell, ops::Deref, rc::Rc};
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

// 1. Drop trait 类似于其他语言中的析构函数，当值离开作用域的时候执行此函数的代码
struct Dog {
    name: String,
    // count: i32,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("{} Dog leave", self.name);
        // self.count -= 1;
    }
}

// 通过Rc<T>允许程序的多个部分之间只读的共享数据，因为相同位置的多个可变引用可能会造成数据竞争和不一样
enum Node {
    Cons(i32, Rc<Node>),
    Nil,
}

use crate::Node::{Cons, Nil};

// 1. 内部可变性： 允许在使用不可变引用时改变数据
// 2. 通过RefCell<T>在运行时检查借用规则，RefCell<T>代表其数据唯一的所有权
// 类似于Rc<T>, RefCell<T>只能用于单线程场景
// 3. 选择Box<T> Rc<T> RefCell<T>的理由
// Rc<T> 允许相同数据有多个所有者；Box<T>和 RefCell<T>有单一所有者
// Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T>允许在运行时执行不可变或可变借用检查
// 因为RefCell<T>允许在运行时执行可变借用检查,所以我们可以在即便 RefCell<T>自身是不可变的情况下修改其内部的值.

#[derive(Debug)]
enum Nodes {
    Nodes(Rc<RefCell<i32>>, Rc<Nodes>),
    Nil,
}

fn main() {
    let b = Box::new(5); // b存储于栈上，5存储在堆上，b指向5所在的内存
    println!("b = {}", b);
    println!("Hello, world!");

    use List::Cons;
    use List::Nil;
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(x, *y);

    let z = Box::new(x);
    assert_eq!(5, *z);

    let y = MyBox::new(x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 将MyBox变为&String, 再将String的解引用变为&str
               // 解引用多态与可变性交互：
               // (1) 当T： Deref<Target=U>时，从&T到&U
               // (2) 当T: DerefMut<Target=U>时，从&mut T 到 &mut U
               // (3) 当T: Deref<Target=U>时，从&mut T到&U

    let a = Dog {
        name: "wangcai".to_string(),
    };
    drop(a); // rust 提供了 std::mem:drop()
    {
        let b = Dog {
            name: "dahuang".to_string(),
        };
        println!("0+++++++++++++++++++++++++++");
    }
    println!("1+++++++++++++++++++++++++++");

    let a = Rc::new(Node::Cons(5, Rc::new(Node::Cons(10, Rc::new(Node::Nil)))));
    println!("count after createing a = {}", Rc::strong_count(&a));
    let b = Node::Cons(3, a.clone());
    println!("count after createing b = {}", Rc::strong_count(&a));
    {
        let c = Node::Cons(4, Rc::clone(&a));
        println!("count after createing c = {}", Rc::strong_count(&a));
    }
    println!("count after leave c = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(crate::Nodes::Nodes(
        value.clone(),
        Rc::new(crate::Nodes::Nil),
    ));
    let b = crate::Nodes::Nodes(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = crate::Nodes::Nodes(Rc::new(RefCell::new(7)), Rc::clone(&a));
    println!("a before: {:?}", a);
    println!("b before: {:?}", b);
    println!("c before: {:?}", c);

    *value.borrow_mut() += 10;
    println!("a after: {:?}", a);
    println!("b after: {:?}", b);
    println!("c after: {:?}", c);
}
