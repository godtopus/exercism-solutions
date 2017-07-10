extern crate gigasecond;

/*
 * Students,
 *
 * Rust does not currently have a library for handling Time. To solve this exercise
 * you'll need to use the Chrono 'crate' (which is Rust's term for an external library).
 *
 * The first time you run `cargo test`, the Chrono crate will automatically be downloaded
 * and installed. More information on crates can be found at
 * https://doc.rust-lang.org/book/guessing-game.html#generating-a-secret-number
 *
 * In order to use the crate, your solution will need to start with the two following lines
*/
extern crate chrono;
use chrono::*;

#[test]
fn test_date() {
    let start_date = UTC.ymd(2011, 4, 25).and_hms(0,0,0);
    assert_eq!(gigasecond::after(start_date), UTC.ymd(2043, 1, 1).and_hms(1,46,40));
}

