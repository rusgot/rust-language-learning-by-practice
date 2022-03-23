use std::ops::Add;

fn main() {
    let meter = Meters(12);
    let millimeter = Millimeters(156);
    let combo = millimeter + meter;
    println!("Combo of millimeters and meters is {:?} millimeters", combo);

    let combo = meter + millimeter;
    println!("Combo of meters and millimeters is {:?} millimeters", combo);
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

    // Using `Output` type here cause why not. Can use `Output` or `Millimeters` interchangably I think
    fn add(self, rhs: Self::Output) -> Self::Output {
        Millimeters((self.0 * 1000) + rhs.0)
    }
}
