// 1. 迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑
// 2. 创建迭代器：迭代器时惰性的，意思就是在调用方法使用迭代器之前，不会有任何效果
// 3. 每个迭代器都实现了 iterator trait, iterator trait 定义在标准库中

// trait iterator {
// type Item;
// fn next(self) -> Option<Self::Item>; // type Item 和 Self::Item 这种用法叫做定义trait的关联类型
// }
// next是Iterator被要求实现的唯一的一个方法，next一次返回一个元素，当迭代器结束时候，返回None
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // 到目前为止，不会对v1产生任何影响
    for val in v1_iter {
        println!("val = {}", val);
    }

    let mut v1_iter = v1.iter();
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v)
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v)
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v)
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v)
    } else {
        println!("at end");
    }

    // 迭代可变引用
    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 3;
    }
    println!("v2 = {:#?}", v2);

    // 消费适配器
    let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum(); // 调用消费适配器sum来求和
    assert_eq!(total, 6);

    // 迭代适配器
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2 = {:?}", v2);

    let v2: Vec<_> = v1.into_iter().filter(|x| *x > 2).collect();
    println!("v2 = {:?}", v2);


    let mut counter = Counter::new();
    for i in 0..6 {
        if let Some(v) = counter.next() {
            println!("i = {}, v = {}", i, v)
        } else {
            println!("i = {}, at end", i);
            break;
        }
    }


}
