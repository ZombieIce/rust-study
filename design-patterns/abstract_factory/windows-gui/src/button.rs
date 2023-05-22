use gui;

pub struct Button;
impl gui::Button for Button {
    fn press(&self) {
        println!("Windows button pressed");
    }
}