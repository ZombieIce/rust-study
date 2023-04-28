trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}
struct PeopleMatchInformation<T, U> {
    master: T,
    student: U,
}

impl<T: GetName + GetAge, U: GetAge + GetName> PeopleMatchInformation<T, U> {
    fn print_all_information(&self) {
        println!("master name = {}", self.master.get_name());
        println!("master age = {}", self.master.get_age());
        println!("student name = {}", self.student.get_name());
        println!("student age = {}", self.student.get_age());
    }
}

struct Master {
    name: String,
    age: u32,
}

impl GetName for Master{
   fn get_name(&self) -> &String {
       &self.name
   } 
}

impl GetAge for Master {
   fn get_age(&self) -> u32 {
       self.age
   } 
}

struct Student{
    name: String,
    age: u32,
}

impl GetName for Student{
   fn get_name(&self) -> &String {
       &self.name
   } 
}

impl GetAge for Student{
   fn get_age(&self) -> u32 {
       self.age
   } 
}

// 对任何实现了特定trait的类型有条件的实现trait
trait PrintName {
    fn print_name(&self);
}

impl <T:GetName> PrintName for T {
    fn print_name(&self) {
        println!("name = {}", self.get_name());
    }
}

fn main() {
    let s = Student{name: "xiaoming".to_string(), age: 15};
    let t = Master{name: "xiaohuang".to_string(), age: 35};
    let m = PeopleMatchInformation{master: t, student: s};
    m.print_all_information();
    println!("Hello, world!");
    let s = Student{name: "xiaoming".to_string(), age: 15};
    s.print_name();
}
