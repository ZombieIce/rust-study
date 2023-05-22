pub struct Button;

use gui;

impl gui::Button for Button {
    fn press(&self) {
        println!("macos button pressed");
    }
}