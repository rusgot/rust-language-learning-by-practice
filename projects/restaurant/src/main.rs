mod front_of_house;

pub use crate::front_of_house::hosting;

fn main() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}
