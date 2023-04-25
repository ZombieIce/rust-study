use std::thread;
use std::time::Duration;

// fn muuuuu(intensity: u32) -> u32 {
//     println!("muuuu......");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn workout(intensity: u32, random_number: u32) {
    // let action = muuuuu;
    let action = || {
        println!("muuuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            // action(intensity)
            action()
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            // action(intensity)
            action()
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            action() // action(intensity)
        );
    }
}

fn main() {
    let intensity = 10;
    let random_number = 7;

    workout(intensity, random_number);

    let s = String::new();
    let update_string = || println!("{}", s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);

    let f = factory;
    let answer = f(1);
}

fn exec<F: FnOnce()>(f: F) {
    f()
}

fn exec1<F: FnMut()>(mut f: F) {
    f()
}

fn exec2<F: Fn()>(f: F) {
    f()
}

fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}
