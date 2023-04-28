// 1. 泛型是具有类型或者其他属性的抽象替代，用于减少代码重复
// 2. 在函数中使用泛型。
// 3. 在结构体中使用泛型
// 4. 枚举中的泛型
// 5. 方法中的泛型
// 6. 总结：使用泛型并不会造成程序性能上的损失。rust通过在编译时进行泛型代码的单态化来保证效率。单态化时通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程

fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter().skip(1) {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

impl <T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 { x: self.x, y: other.y }
    }
}

fn main() {
    let number_list = vec![1, 2, 3, 33, 22, 31];
    let max_number = largest(&number_list);
    println!("{}", max_number);

    let char_list = vec!['a', 'c', 'A'];
    let max_char = largest(&char_list);
    println!("max_char = {}", max_char);

    let integer = Point{x: 1, y: 2};
    println!("{:#?}", integer);

    let float = Point{x:1.1, y:2.2};
    println!("{:?}", float);
    
    let a = Point2{x: 1.1, y: 'c'};
    println!("{:#?}", a);

    println!("{}", integer.get_x());
    println!("{}", integer.get_y());
    println!("{:#?}", integer);

    let p1 = Point2{x: 5, y: 1.3};
    let p2 = Point2{x: "hello", y: 'c'};
    let mix_p = p1.mixup(p2);
    println!("{:#?}", mix_p);
}
