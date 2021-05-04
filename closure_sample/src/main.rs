
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Copy + Hash + Eq,
          V: Copy
{
    calculation: T,
    value: HashMap<U, V>
}

impl<T, U, V> Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Copy + Hash + Eq,
          V: Copy
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        *self.value.entry(arg).or_insert((self.calculation)(arg))
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

#[test]
fn other_test1() {
    let x = 4;

    let equal_to_x = move |z| { z == x };

    assert_eq!(x, 4);

    let y = 4;

    assert!(equal_to_x(y));
}

#[test]
fn other_test2() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // ここでは、xを使用できません: {:?}
    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}