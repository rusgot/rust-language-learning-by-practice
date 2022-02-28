use jake_crates_io_test::kinds::PrimaryColor;
use jake_crates_io_test::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    let result = mix(red, yellow);
    println!("{:?}", result);
}
