fn move_coords(x: (i32, i32)) -> (i32, i32) {
    (x.0+1, x.1+1)
}

#[derive (Debug, PartialEq)] // 允许对结构体实例进行打印和比较
struct People {
    name: &'static str,
    gender: u32,
}

impl People {
    fn new(name: &'static str, gneder: u32) -> Self {
        return People { name: name , gender: gneder }
    }

    fn name(&self) {
        println!("name: {:?}", self.name);
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn gender(&self) {
        let gender = if self.gender == 1 {"boy"} else {"girl"};
        println!("gender: {:?}", gender);
    }
}

struct Integer(u32);
type Int = i32;

struct Color(i32, i32, i32);

struct Empty;

enum Number {
    Zero,
    One,
    Two,
}

fn main() {
    let tuple: (&str, i32, char) = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
    let coords = (0, 1);
    let result = move_coords(coords);
    assert_eq!(result, (1, 2));
    let (x, y) = move_coords(coords);
    assert_eq!(x, 1);
    assert_eq!(y, 2);

    let alex = People::new("Alex", 1);
    alex.name();
    alex.gender();
    assert_eq!(alex, People {name: "Alex", gender: 1});
    let mut alice = People::new("Alice", 0);
    alice.name();
    alice.gender();
    alice.set_name("Rose");
    alice.name();
    assert_eq!(alice, People{name: "Rose", gender: 0});

    let int = Integer(10);
    assert_eq!(int.0, 10);
    let int: Int = 10;
    assert_eq!(int, 10);

    let color = Color(0, 1, 2);
    assert_eq!(color.0, 0);
    assert_eq!(color.1, 1);
    assert_eq!(color.2, 2);

    let x = Empty;
    println!("{:p}", &x);
    let y = x;
    println!("{:p}", &y);
    let z = Empty;
    println!("{:p}", &z);
    assert_eq!((..), std::ops::RangeFull);

    let a = Number::One;
    match a {
        Number::Zero => println!("0"),
        Number::One => println!("1"),
        Number::Two => println!("2"),
    }

    let s: &Option<String> = &Some("hello".to_string());
    match s {
        Some(s) => println!("s is: {}", s),
        _ => (),
    }
}
