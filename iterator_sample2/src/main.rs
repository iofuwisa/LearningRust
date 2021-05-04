struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn iterator_collect() {
    let mut c = Counter::new();

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    assert_eq!(v2, vec![2, 3, 4]);
}
