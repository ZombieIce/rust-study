struct Counter {
    count: u32,
}

impl Counter {
    fn new()->Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}


fn main() {
    let values = vec![1, 2, 3];
    for v in values.into_iter() {
        println!("{}", v);
    }

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    let mut value_iter_mut = values.iter_mut();
    if let Some(v) = value_iter_mut.next() {
        *v = 0;
    }

    println!("{:?}", values);

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter()
    .enumerate()
    .filter(|&(idx, _)| idx % 2 ==0)
    .map(|(_, val)| val)
    .fold(0u64, |sum, acm| sum + acm);
    println!("{}", val);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
