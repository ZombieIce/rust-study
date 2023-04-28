// 对象、封装、继承
// 对象：数据和操作数据的过程
// Rust里面，结构体、枚举类型加上Impl

struct Dog {
    name: String,
}

impl Dog {
    fn print_name(&self) {
        println!("Dog name = {}", self.name);
    }
}


fn main() {
    let d = Dog{name: "wangcai".to_string()};
    d.print_name();
    println!("Hello, world!");
}
