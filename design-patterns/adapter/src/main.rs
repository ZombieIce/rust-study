mod adaptee;
mod target;
mod adapter;

use adaptee::SpecificTarget;
use adapter::TargetAdapter;
use target::{Target, OrdinaryTarget};


fn call(target: impl Target) {
    println!("{}", target.request());
}

fn main() {
    let target = OrdinaryTarget;
    print!("A compatible target can be directly used: ");
    call(target);

    let adaptee = SpecificTarget;
    println!("Adaptee is incompatible with client: '{}'", adaptee.specific_request());

    let adapter = TargetAdapter::new(adaptee);
    print!("But the adapter can be used instead: ");
    call(adapter);
}
