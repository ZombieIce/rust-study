use core::panic;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

struct Factory {
    factory_id: usize,
}

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

fn generate_id() -> usize {
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory ids overflowed");
    }
    let next_id = GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overflowed");
    }
    next_id
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }
}

static NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));

fn main() {
    let v = NAMES.lock().unwrap();
    println!("{}", v);
}
