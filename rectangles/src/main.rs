#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
impl Rectangle {
    fn set_line(&mut self, w: u32, h: u32) {
        self.width =w;
        self.height = h;
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn gaisyuu(&self) -> u32 {
        self.width * 2 + self.height * 2
    }
}

fn main() {
    let mut rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The gausyuu of the rectangle is {} square pixels.",
        rect1.gaisyuu()
    );

    rect1.set_line(20, 25);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The gausyuu of the rectangle is {} square pixels.",
        rect1.gaisyuu()
    );
}