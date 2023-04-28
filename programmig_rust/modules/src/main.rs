mod user_info;
use user_info::user::User;

mod say {
    pub fn hello() {
        println!("Hello, world!");
    }

    fn hello_2() {
        println!("hello");
    }

    pub mod hi {
        pub fn hi_1() {
            super::hello_2();
        }

        pub fn hi_2() {
            println!("hi there");
        }
    }
}


fn main() {
    // 相对路径
    say::hello();
    // 绝对路径调用
    crate::say::hello();

    say::hi::hi_1();
    say::hi::hi_2();

    let u1 = User::new_user(String::from("tom"), 5);
    println!("user name: {}", u1.name());
    println!("1+2: {}", user_info::user::add(1, 2));
}
