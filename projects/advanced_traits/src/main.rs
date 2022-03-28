use std::fmt;
use std::ops::Add;

fn main() {
    let mut counter = Counter::new();

    println!("Original count: {}", counter.count);

    for _ in 0..9 {
        let new_count = counter.next();
        println!("New count: {}", new_count.unwrap());
    }

    let mut counter_2 = Counter::new();
    counter_2.next();
    counter_2.next(); // Should be 2 now

    let counter_3 = counter + counter_2;
    println!("Counter 3 count is {}", counter_3.count); // Should be 11

    let meter = Meters(12);
    let millimeter = Millimeters(156);
    let combo = millimeter + meter;
    println!("Combo of millimeters and meters is {:?} millimeters", combo);

    let combo = meter + millimeter;
    println!("Combo of meters and millimeters is {:?} millimeters", combo);

    let jake = Human;
    jake.fly();
    Pilot::fly(&jake);
    Wizard::fly(&jake);

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let point = Point { x: 8, y: 12 };

    point.outline_print();

    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("{}", w);

    let res = type_alias_test();
}

type Result = std::result::Result<i32, f32>;

fn type_alias_test() -> Result {
    Err(420.69)
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// Can only implement `OutlinePrint` trait for a type if that type already implements the `Display` trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*flapping arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("I conjure the winds of a forgotten age, carry me to the heavens!");
    }
}

#[derive(Debug, Copy, Clone)]
struct Meters(u32);

#[derive(Debug, Copy, Clone)]
struct Millimeters(u32);

// This impl allows Meters to be added to Millimeters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// This impl allows Millimeters to be added to Meters
impl Add<Millimeters> for Meters {
    type Output = Millimeters;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Millimeters((self.0 * 1000) + rhs.0)
    }
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: <Counter as Iterator>::Item,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count + 1;
        Some(self.count)
    }
}

// Implement the ability to add two Counters together (use them with the + operator)
impl Add for Counter {
    type Output = Counter;

    fn add(self, rhs: Counter) -> Self::Output {
        Counter {
            count: self.count + rhs.count,
        }
    }
}
