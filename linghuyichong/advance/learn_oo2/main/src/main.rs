use gui::{Button, Screen, SelectBox};

fn main() {
    // 使用trait对象时，Rust必须使用动态分发。编译器无法知晓所有可能用于trait对象代码的类型
    // 所以它也不知道应该调用哪个类型的哪个方法。为此，Rust在运行时使用 trait 对象中的指针来知晓
    // 需要调用哪个方法
    let s = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: "ok".to_string(),
            }),
            Box::new(SelectBox {
                width: 60,
                height: 40,
                options: vec!["yes".to_string(), "no".to_string(), "maybe".to_string()],
            }),
        ],
    };

    s.run();

    println!("Hello, world!");
}
