mod render;
use gui::GuiFactoryDynamic;
use macos_gui::factory::MacosFactory;
use windows_gui::factory::WindowsFactory;

fn main() {
    let windows = true;
    if windows {
        render::render_static(&WindowsFactory);
    } else {
        render::render_static(&MacosFactory);
    }

    let windows = false;
    let factory: &dyn GuiFactoryDynamic = if windows {
        &WindowsFactory
    } else {
        &MacosFactory
    };

    render::render_dynamic(factory);
}
