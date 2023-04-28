// 函数指针 fn被称为函数指针
// 函数指针实现了 Fn FnMut FnOnce
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, val: i32) -> i32 {
    f(f(val))
}

fn wapper_func<T>(t: T, v: i32) -> i32 
where T: Fn(i32)->i32{
    t(v)
}

fn func(v: i32) -> i32 {
    v + 1
}

// 返回闭包
fn return_clo() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x+1)
}

fn main() {
    let r = do_twice(add_one, 5);
    assert_eq!(r, 7);
    println!("Hello, world!");

    let a = wapper_func(|x|x+1, 1);
    println!("a = {}", a);

    let a = wapper_func(func, 1);
    println!("a = {}", a);

    let c = return_clo();
    println!("1 + 1 = {}", c(1));
    println!("1 + 1 = {}", (*c)(1));
}
