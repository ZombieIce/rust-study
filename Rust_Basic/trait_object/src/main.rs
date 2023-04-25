pub trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw1(x: Box<dyn Draw>) {
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("draw Button at {} {}", self.width, self.height)
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        format!("draw SelectBox at {} {}", self.width, self.height)
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

fn main() {
    let x = 1.1f64;
    let y = 8u8;
    draw1(Box::new(x));
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}
