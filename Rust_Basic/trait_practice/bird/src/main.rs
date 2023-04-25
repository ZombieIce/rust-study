trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is siwmming")
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn main() {
    let duck = Duck {};
    duck.swim();

    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");

    let birds :Vec<Box<dyn Bird>>= vec![Box::new(Duck), Box::new(Swan)];
    for bird in birds {
        println!("{}", bird.quack());
    }
}

fn hatch_a_bird(n: i32) -> Box<dyn Bird> {
    match n {
        1 => Box::new(Swan),
        _ => Box::new(Duck),
    }
}
