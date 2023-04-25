use std::rc::Rc;
use std::thread;
use std::sync::Arc;

struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}

struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!");
    }
}

struct Owner {
    name: String,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}


fn main() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let _foo = Foo;
    println!("Running!");

    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));


    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


    let gadget_owner: Rc<Owner> = Rc::new(Owner { name: "Gadget Man".to_string()});
    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner)
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner)
    };

    drop(gadget_owner);

    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            println!("{}", s)
        });
    }

}
