use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
/*
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}*/

// Defining a Cacher struct that holds a closure in calculation and an optional result in value
struct Cacher<T, Key, Value: Copy>
    where T: Fn(Key) -> Value,
          Key: Eq + Hash + Clone + Debug + Copy
{
    calculation: T,
    value: HashMap<Key, Value>
}

impl<T, Key, Value: Copy> Cacher<T, Key, Value>
    where T: Fn(Key) -> Value,
          Key: Eq + Hash + Clone + Debug + Copy
{
    fn new(calculation: T) -> Cacher<T, Key, Value> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: Key) -> Value {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);// call the closure and pass to it the argument arg
                self.value.insert(arg, v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_result = Cacher::new(
        |num| {
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
                "Today, run for {} minutes! s",
                expensive_result.value(100)
            );

        }
    }
}


fn main() {
    let simulated_user_specified_value = 28;
    let simulated_random_number = 8;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
