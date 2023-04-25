use std::io;

fn math(op: fn(i32, i32)-> i32, a: i32, b: i32) ->i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(& mut guess).expect("Failed to read line");
    println!("You guessed: {guess}");

    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", other);
    let other = place2;
    println!("{:?}", other);


    let a = 2;
    let b = 3;
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);
}
