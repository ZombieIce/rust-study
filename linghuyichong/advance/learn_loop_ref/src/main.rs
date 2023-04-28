use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
enum Nodes {
    Node(i32, RefCell<Rc<Nodes>>),
    Nil,
}

impl Nodes {
    fn tail(&self) -> Option<&RefCell<Rc<Nodes>>> {
        match self {
            Nodes::Node(_, item) => Some(item),
            Nodes::Nil => None,
        }
    }
}

// 弱引用Weak<T>
// 弱引用通过Rc::downgrade传递Rc示例的引用，调用Rc::downgrade会得到Weak<T>
// 区别在于 weak_count 无需计数为0就能使 Rc 实例被清理
// 可以通过 Rc::upgrade 方法返回 Option<Rc<T>>对象

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

use crate::List::Nil;

fn main() {
    // let a = Rc::new(Nodes::Node(5, RefCell::new(Rc::new(Nodes::Nil))));
    // println!("1, a rc count = {}", Rc::strong_count(&a));
    // println!("1, a tail = {:?}", a.tail());

    // let b = Rc::new(Nodes::Node(10, RefCell::new(Rc::clone(&a))));
    // println!("2, a rc count = {}", Rc::strong_count(&a));
    // println!("2, b rc count = {}", Rc::strong_count(&b));
    // println!("2, b tail = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("3, a rc count = {}", Rc::strong_count(&a));
    // println!("3, b rc count = {}", Rc::strong_count(&b));


    let na = Rc::new(List::Cons(5, RefCell::new(Weak::new())));
    println!("1, a strong count = {}, weak count = {}", Rc::strong_count(&na), Rc::weak_count(&na));
    println!("1, a tail = {:?}", na.tail());

    let nb = Rc::new(List::Cons(10, RefCell::new(Weak::new())));
    if let Some(link) = nb.tail() {
        *link.borrow_mut() = Rc::downgrade(&na);
    }

    println!("2, a rc count = {}, weak count = {}", Rc::strong_count(&na), Rc::weak_count(&na));
    println!("2, b rc count = {}, weak count = {}", Rc::strong_count(&nb), Rc::weak_count(&nb));
    println!("2, b tail = {:?}", nb.tail());

    if let Some(link) = na.tail() {
        *link.borrow_mut() = Rc::downgrade(&nb);
    }
    println!("3, a rc count = {}, weak count = {}", Rc::strong_count(&na), Rc::weak_count(&na));
    println!("3, b rc count = {}, weak count = {}", Rc::strong_count(&nb), Rc::weak_count(&nb));
    println!("3, b tail = {:?}", nb.tail());


}
