// dynamic dispatch
use std::fmt::Debug;

trait Drawable {
    fn draw(&self);
}

trait DrawableDebug: Drawable + Debug {}

#[derive(Debug)]
struct One;

impl Drawable for One {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

impl DrawableDebug for One {}

// 特征对象只能包含一个非自动特征,非 auto trait
fn draw_and_debug(item: &dyn DrawableDebug) {
    item.draw();
    println!("{:?}", item);
}

fn main() {
    let one = One;
    draw_and_debug(&one);
}



use std::fmt::Debug;

trait Drawable {
    fn draw(&self);
}

// 定义一个组合特征，继承 Drawable 和 Debug
trait DrawableDebug: Drawable + Debug {}

impl<T: Drawable + Debug> DrawableDebug for T {}

#[derive(Debug)]
struct One;

impl Drawable for One {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

// 使用组合特征对象
fn draw_and_debug(item: &dyn DrawableDebug) {
    item.draw();
    println!("{:?}", item);
}

fn main() {
    let one = One;
    draw_and_debug(&one); // 通过特征对象的动态分发来调用方法
}



// 编译期静态处理
use std::fmt::Debug;

trait Drawable {
    fn draw(&self);
}
trait DrawableDebug: Drawable + Debug {}
#[derive(Debug)]
struct One;

impl Drawable for One {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

// impl DrawableDebug for One {}
impl<T: Drawable + Debug> DrawableDebug for T {}

fn draw_and_debug<T: DrawableDebug>(item: &T) {
    item.draw();
    println!("{:?}", item);
}

fn main() {
    let one = One;
    draw_and_debug(&one);
}
