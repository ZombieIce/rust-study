pub struct Checkbox;

use gui;

impl gui::Checkbox for Checkbox {
    fn switch(&self) {
        println!("Windows checkbox switched");
    }
}