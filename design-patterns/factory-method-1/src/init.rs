use crate::gui::Dialog;
use crate::html_gui::HtmlDialog;
use crate::windows_gui::WindowsDialog;

pub fn initialize() -> &'static dyn Dialog {
    if cfg!(target_os = "windows") {
        println!("-- Windows detected, creating Windows GUI --");
        return &WindowsDialog;
    } else {
        println!("-- No OS detected, creating HTML GUI --");
        return &HtmlDialog;
    }
}
