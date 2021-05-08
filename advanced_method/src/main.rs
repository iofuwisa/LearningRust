fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(move |mut x:i32| -> i32 {x += 1;x})
}

fn main() {
    let answer = do_twice(add_one, 5);

    let c = returns_closure();

    let x = 10;
    let x2= c(x);

    // 答えは{}
    println!("The answer is: {}", answer);
    println!("{}",x);
    println!("{}",x2);
}