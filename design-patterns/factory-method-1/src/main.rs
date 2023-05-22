mod gui;
mod html_gui;
mod windows_gui;
mod init;

use init::initialize;

fn main() {
    let dialog = initialize();
    dialog.render();
    dialog.refresh();
}
