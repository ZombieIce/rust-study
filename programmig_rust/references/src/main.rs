use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {};", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn show_ref(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

struct Point {
    x: i32,
    y: i32,
}

static mut STASH: &i32 = &128;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest<'a>(v: &'a [i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

struct S<'a, 'b> {
    x: &'a i32,
    y: &'b i32
}

fn sum_r_xy(r: &i32, s: S) -> i32 {
    r + s.x + s.y
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    show_ref(&table);
    println!("=================");
    sort_works(&mut table);
    show(table);

    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);

    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);
    assert_eq!(rrr.x, 1000);

    f(unsafe { STASH });

    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S{x: &x, y: &y};
            r = sum_r_xy(&30, s);
        }
    }
    println!("{}", r);

    let vs:StringTable = StringTable { elements: vec!["aa123".to_string(), "bb".to_string()] };
    println!("{}", vs.find_by_prefix("aa").unwrap());

    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    // x += 10; cannot assign to 'x' because it is borrowed
    // let m = &mut x; cannot borrow 'x' as mutable because it is also borrowd as immutable
    println!("{}, {}", r1, r2);
}
