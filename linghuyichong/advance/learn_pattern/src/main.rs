// 模式用来匹配值的结构
// 字面值，结构的数组、枚举、结构体或元组， 变量， 通配符， 占位符

struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let a = 1;
    // match 必须匹配完所有情况
    match a {
        0 => println!("ZERO"),
        1 => println!("ONE"),
        _ => println!("other"),
    }

    let color: Option<String> = Some("red".to_string());
    let is_ok = true;
    let age: Result<u8, _> = "33".parse();

    if let Some(c) = color {
        println!("color: {}", &c);
    } else if is_ok {
        println!("is ok")
    } else if let Ok(a) = age {
        if a > 30 {
            println!("oh, mature man");
        } else {
            println!("oh, young man");
        }
    } else {
        println!("in else")
    }

    println!("Hello, world!");

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("top = {}", top);
    }

    // for 循环中，模式是直接跟随for关键字的值，例如 for x in y, x 就是对应的模式
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }

    // 函数的参数也是模式
    fn print_point(&(x, y): &(i32, i32)) {
        println!("x: {}, y: {}", x, y);
    }

    let p = (3, 5);
    print_point(&p);

    // 模式有两种， 可反驳的 不可反驳
    // 只能接受不可反驳模式的有 函数、let语句、for循环
    // if let 和 while let 只能接受可反驳的模式
    let a: Option<i32> = Some(5); // 匹配Some(value), None
    let b: Option<i32> = None;
    if let Some(x) = a {
        println!("x {}", x);
    }

    // 匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }

    // 匹配命名变量
    let x = Some(5);
    let y = 10; // 位置1
    match x {
        Some(50) => println!("50"),
        Some(y) => println!("y = {}", y), // 此处的y不是位置1的y
        _ => println!("other"),
    };

    println!("x = {:?}, y = {:?}", x, y); // 此处的y是位置1的

    // 多个模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // 通过..匹配
    let x = 5;
    match x {
        1..=5 => println!("one to five"),
        _ => println!("other"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("1"),
        'k'..='z' => println!("2"),
        _ => println!("other"),
    }

    // 解构并分解值 元组、结构体、枚举、引用
    let p = Point { x: 1, y: 2 };
    // 变量 a和变量b匹配 x 和 y
    let Point { x: a, y: b } = p;
    assert_eq!(1, a);
    assert_eq!(2, b);
    let Point { x, y } = p;
    assert_eq!(1, x);
    assert_eq!(2, y);

    match p {
        Point { x, y: 2 } => println!("x axis"),
        Point { x: 0, y } => println!("y axis"),
        _ => println!("other"),
    }

    let numbers = (1, 2, 3, 4);
    match numbers {
        (one, _, three, _) => {
            println!("one = {}, three = {}", one, three)
        }
    }

    // _ 编译器忽略警告
    let _x = 5;
    let _y = 5;
    let s = Some(String::from("hello"));
    // 同样会发生值的绑定
    if let Some(_c) = s {
        println!("found a string");
    }

    let s = Some(String::from("hello"));
    if let Some(_) = s {
        println!("found _ string");
    }

    let v = (1, 2, 3, 4, 5, 6);
    match v {
        (first, .., last) => {
            println!("first: {}, last: {}", first, last);
        }
    }
}

// 忽略模式中的值
fn foo(_: i32, y: i32) {
    println!("y = {}", y);
}

trait A {
    fn bar(x: i32, y: i32);
}

struct B {}
impl A for B {
    fn bar(_: i32, y: i32) {
        println!("y = {}", y);
    }
}
