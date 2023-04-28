trait GetName {
    fn get_name(&self) -> &String;
}
trait GetAge {
    fn get_age(&self) -> u32;
}

fn print_information<T: GetAge + GetName>(item: T) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

fn print_information_where<T>(item: T)
where T: GetAge + GetName {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
}
impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}
impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

fn produce_item_with_age() -> impl GetAge {
    Student {
        name: String::from("xiaoming"),
        age: 15,
    }
}


fn main() {
    let s = Student{name: String::from("xiaozhang"), age: 18};
    // print_information(s);
    print_information_where(s);

    let s = produce_item_with_age();
    println!("{:#?}", s.get_age());
}
