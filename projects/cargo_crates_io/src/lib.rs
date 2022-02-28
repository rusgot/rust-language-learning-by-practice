//! # My Crate
//!
//! `cargo_crates_io` is me learning/practicing the capabilities of `cargo`.

/// Adds two to the number given.
///
/// # Examples
///
/// ```
/// let arg = 7;
/// let answer = cargo_crates_io::add_two(arg);
///
/// assert_eq!(9, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}
