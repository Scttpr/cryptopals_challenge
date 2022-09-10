#![warn(clippy::nursery, clippy::pedantic, nonstandard_style)]

mod set_1;
mod tools;

use tools::display::{challenge, set, title};

fn main() {
    title();

    set(1);
    challenge(1, set_1::challenge_1::resolve);
    challenge(2, set_1::challenge_2::resolve);
    challenge(3, set_1::challenge_3::resolve);
    challenge(4, set_1::challenge_4::resolve);
    challenge(5, set_1::challenge_5::resolve);
    challenge(6, set_1::challenge_6::resolve);
}
