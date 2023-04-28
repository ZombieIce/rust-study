// 1. 解引用裸指针
// 2. 调用不安全的函数或者方法
// 3. 访问或修改可变静态变量
// 4. 实现不安全的trait

// 调用不安全的函数或者方法
unsafe fn dangerous() {
    println!("do something dangerous");
}

fn foo() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = & mut num as *mut i32;
    unsafe {
        println!("*i1 = {}", *r1);
        println!("*i2 = {}", *r2);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

// 静态变量和常量的区别：
// 静态变量有一个固定的内存地址（使用这个值总会访问相同的地址），常量则允许在任何被用到的时候复制其数据
// 静态变量可以是可变的，虽然这可能是不安全(用 unsafe 包含)
static HELLO_WORLD: &str = "Hello, World";
static mut COUNTER: u32 = 0;

fn add_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 实现不安全的trait
// 1. 当至少有一个方法中包含编译器不能验证的不变量时，该trait就是不安全的
// 2. 在 trait 之前增加 unsafe 声明其为不安全的，同时 trait 的实现也必须用 unsafe 标记
unsafe trait Foo {
    fn foo(&self);
}

struct Bar();
unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("foo bar");
    }
}

fn main() {
    // 解引用指针 可变和不可变的 分别写作 *const T, *mut T
    // 1. 允许忽略借用规则，同时可以拥有不可变和可变的指针，或者是多个指向相同位置的可变指针
    // 2. 不保证只想的内存是有效的
    // 3. 允许为空
    // 4. 不能实现任何自动清理的功能
    let mut num = 5;
    // 创建不可变和可变的裸指针可以在安全的代码块中，只是不能在不安全代码块之中使用
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let add = 0x12345usize;
    let _r = add as *const i32;

    unsafe {
        dangerous();
    }
    foo();

    unsafe {
        println!("abs(-3): {}", abs(-3));
    }

    println!("{}", HELLO_WORLD);

    add_counter(3);
    add_counter(3);
    unsafe {
        println!("counter: {}", COUNTER);
    }

    let a = Bar();
    a.foo();

}
