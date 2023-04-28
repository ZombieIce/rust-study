struct Person { name: String, birth: i32}

fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    // let t = s.clone();
    // let u = s.clone();
    
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // value "Govinda" dropped here

    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string(); // nothing is dropped here
    println!("Hello, world!");

    let mut composers = Vec::new();
    composers.push(Person{name: "Palestrina".to_string(), birth: 1525});

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // Pop a value off the end of the vector
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // Move a value out of a given index in the vector, and move the last element into its spot
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // Swap in another value for the one we're takign out
    let third = std::mem::replace(&mut v[2], "subsititute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "substitute"]);

}
