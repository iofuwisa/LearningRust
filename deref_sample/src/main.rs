use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y1 = &x;
    let y2 = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y1);
    assert_eq!(5, *y2);



    let mut a = 0;
    let b = &mut a;

    *b += 1;

    println!("b : {}", b);
    println!("a : {}", a);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let m = MyBox::new(String::from("Last"));
    hello(&(*m));

}