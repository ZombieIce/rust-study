pub struct MacosFactory;

use gui;
use crate::button;
use crate::checkbox;

impl gui::GuiFactory for MacosFactory {
    type B = button::Button;
    type C = checkbox::Checkbox;

    fn create_button(&self) -> Self::B {
        button::Button
    }

    fn create_checkbox(&self) -> Self::C {
        checkbox::Checkbox
    }
}

impl gui::GuiFactoryDynamic for MacosFactory {
    fn create_button(&self) -> Box<dyn gui::Button> {
        Box::new(button::Button)
    }

    fn create_checkbox(&self) -> Box<dyn gui::Checkbox> {
        Box::new(checkbox::Checkbox)
    }
}