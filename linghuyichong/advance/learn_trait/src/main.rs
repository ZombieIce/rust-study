// 1. trait 用于定义与其他类型共享的功能，类似于其他语言中的接口
// (1) 可以通过trait以抽象的方式定义共享的行为
// (2) 可以使用trait bounds 指定泛型是任何拥有特定行为的类型
// 2. 定义trait

trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

// 默认实现
trait SchoolName {
    fn get_school_name(&self) -> String {
        String::from("Hongxing Shool")
    }
}

pub struct Student {
    pub name: String,
    pub age: u32,
}

impl SchoolName for Student {}

impl GetInformation for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl SchoolName for Teacher {
    fn get_school_name(&self) -> String {
        String::from("Guangming School")
    }
}

impl GetInformation for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

// 5. trait 作为参数 直接作为参数的写法
fn print_information(item: impl GetInformation) {
    println!("name = {}, age = {}", item.get_name(), item.get_age());
}

// 使用trait bound的写法
fn print_new_information<T: GetInformation>(item: T) {
    println!("name = {}, age = {}", item.get_name(), item.get_age());
}

fn print_two_information<T: GetAge + GetName>(item: T) {
    println!("name = {}, age = {}", item.get_name(), item.get_age());
}

fn main() {
    let s = Student {
        name: "xiaoming".to_string(),
        age: 10,
    };
    let t = Teacher {
        name: "xiaohuang".to_string(),
        age: 30,
        subject: "english".to_string(),
    };
    println!("student name = {}, age = {}", GetInformation::get_name(&s), GetInformation::get_age(&s));
    println!("teacher name = {}, age = {}", t.get_name(), t.get_age());

    let s_school_name = s.get_school_name();
    println!("student school name = {}", s_school_name);
    let t_school_name = t.get_school_name();
    println!("teacher school name = {}", t_school_name);

    print_information(s);
    print_new_information(t);
}
