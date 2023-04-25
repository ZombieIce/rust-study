fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}

fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let a = 2;
    let b = 3;
    assert_eq!(math(add, a, b), 5);
    assert_eq!(math(product, a, b), 6);

    let result = two_times_impl();
    assert_eq!(result(2), 4);

    let n = 13;
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    assert_eq!(big_n, 6);

    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let number = 42;
    match number {
        0 => println!("Origin"),
        1..=3 => println!("ALL"),
        5 | 7 | 13 => println!("Bad Luck"),
        n @ 42 => println!("Answer is {}", n),
        _ => println!("Common"),
    }

    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        // 左侧为模式，右侧为要匹配的值
        binary = 1;
    }
    assert_eq!(binary, 1);

    let mut v = vec![1, 2, 3, 4, 5];
    loop {
        // 调用v的pop方法会返回Option类型，所以用match匹配两种情况，Some(x)和None
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) = v.pop() { // Some(x)为匹配模式，它会匹配右侧pop方法调用返回的Option类型结果
        println!("{}", x);
    }


}
