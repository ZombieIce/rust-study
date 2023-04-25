fn main() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.len(), 3);
    println!("capacity is now {}", v.capacity());

    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v;

    print(&a);
    print(&v);
    print(&v[0..2]);
    print(&a[2..]);
    print(&sv[1..3]);
}

fn print(n: &[f64]) {
    for elt in n {
        print!("{} ", elt);
    }
    println!("");
}