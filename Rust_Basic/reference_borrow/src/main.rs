fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let ss = String::from("hello");
    let r1 = &ss;
    let r2 = &ss;
    println!("{}, {} and {}", ss, r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
