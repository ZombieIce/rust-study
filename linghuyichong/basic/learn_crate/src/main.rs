use mylib::factory::product_washing_machine;

mod mod_a {
    #[derive(Debug)]
    pub struct A {
        pub number: i32,
        name: String,
    }

    impl A {
        pub fn new_a() -> A {
            A {
                number: 1,
                name: String::from("A"),
            }
        }

        pub fn print_a(&self) {
            println!("number: {}, name: {}", self.number, self.name);
        }
    }

    pub mod mod_b {
        pub fn print_b() {
            println!("B");
        }

        pub mod mod_c {
            pub fn print_c() {
                println!("C");
                super::print_b();
            }
        }
    }
}

use mod_a as A;
fn main() {
    mylib::factory::produce_refrigerator::produce_re(); // absolute path
    product_washing_machine::produce_washing_machine(); // use


    let a = mod_a::A::new_a();
    a.print_a();
    // let number = a.number;
    // let name = a.name;
    A::mod_b::mod_c::print_c();
}
