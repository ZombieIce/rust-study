struct  Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn display_arrays<T: std::fmt::Debug, const N:usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());

    let arr: [i32; 3] = [1, 2, 3];
    display_arrays(arr);

    let arr: [i32; 2] = [1, 2];
    display_arrays(arr);
}
