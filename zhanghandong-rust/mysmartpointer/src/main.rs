use std::ops::Deref;

struct MySmartPointer<T>(T);

impl <T> MySmartPointer<T> {
    fn new(x:T) -> MySmartPointer<T> {
        MySmartPointer(x)
    }
}

impl <T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MySmartPointer::new(x);

    assert_eq!(5, x);
    assert_eq!(x, *y);
}
