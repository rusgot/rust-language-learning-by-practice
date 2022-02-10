use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let mut cacher: Cacher<_, u32, u64> = Cacher::new(|x| (x * 2) as u64);
    println!("val: {}", cacher.value(3));
    println!("val: {}", cacher.value(3));
    println!("val: {}", cacher.value(6));
    println!("val: {}", cacher.value(6));
    println!("val: {}", cacher.value(3));
}

struct Cacher<T, U, Z>
where
    T: Fn(U) -> Z,
    U: std::cmp::Eq + std::hash::Hash,
{
    calculation: T,
    value: HashMap<U, Z>,
}

impl<T, U, Z> Cacher<T, U, Z>
where
    T: Fn(U) -> Z,
    U: std::cmp::Eq + std::hash::Hash + std::marker::Copy,
    Z: std::marker::Copy,
{
    fn new(calculation: T) -> Cacher<T, U, Z> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> Z {
        match self.value.contains_key(&arg) {
            true => {
                println!("Value already stored, no need to reclalculate");
                *self.value.get(&arg).unwrap()
            }
            false => {
                println!("Value never calculated, calculating now...");
                let dur = Duration::new(2, 0);
                thread::sleep(dur);
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}
