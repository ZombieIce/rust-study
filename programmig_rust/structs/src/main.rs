use std::cell::RefCell;
use std::rc::Rc;

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}
#[derive(Clone, Copy)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}

pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

struct Node {
    tag: String,
    children: Vec<Rc<Node>>,
}

pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };

    fn scale(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
    }
}

impl Node {
    fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![],
        }
    }

    fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least { least = &slice[i];}
        if slice[i] > *greatest { greatest = &slice[i];}
    }
    Extrema { greatest: greatest, least: least }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}


fn main() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);

    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('8');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('8'));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('2');
    assert!(!q.is_empty());

    q.push('x');
    assert_eq!(q.pop(), Some('2'));
    q.push('D');
    let (older, younger) = q.split();
    assert_eq!(older, vec!['x']);
    assert_eq!(younger, vec!['D']);

    let mut parent = Node::new("parent");
    let shared_node = Rc::new(Node::new("first"));
    let shared_node_1 = shared_node.clone();
    shared_node.append_to(&mut parent);
    shared_node_1.append_to(&mut parent);
    for n in parent.children.iter() {
        println!("{}", n.tag)
    }

    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.greatest, 48);
    assert_eq!(*e.least, -3);
    for i in a.iter() {
        println!("{}", *i);
    }

    let ref_cell: RefCell<String> = RefCell::new("hello".to_string());
    let r = ref_cell.borrow();
    let count = r.len();
    assert_eq!(count, 5);

    // let mut w = ref_cell.borrow_mut(); // panic: already borrowed
    // w.push_str(" world");


}
