extern crate oop_sample;
use oop_sample::{Draw, Screen, Button};

use std::fmt::Display;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("selectbox drawn width: {} height: {} label {:?}", self.width, self.height, self.options);
    }
}

struct DrawableValue<T> {
    value: Box<T>
}

impl<T> Draw for DrawableValue<T> where T: Display{
    fn draw(&self) {
        println!("value drawn value: {}", self.value);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    // はい
                    String::from("Yes"),
                    // 多分
                    String::from("Maybe"),
                    // いいえ
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                // 了解
                label: String::from("OK"),
            }),
            Box::new(DrawableValue{
                value: Box::new(
                    String::from("hello woorld"),
                )
            }),
        ],
    };

    screen.run();
}