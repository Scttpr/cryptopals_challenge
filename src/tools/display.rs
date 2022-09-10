use colored::Colorize;
use std::fmt::Debug;

pub fn title() {
    println!(
        "\n{}\n",
        ">> CRYPTOPALS CHALLENGE <<".to_string().bold().green()
    );
}

pub fn set(index: u8) {
    println!("{}", format!(">> SET {}", index).bold().bright_yellow());
}

pub fn challenge<T: Debug>(index: u8, resolver: fn() -> T) {
    println!("{}", format!("-- Challenge {} --", index).bold());

    let output = resolver();

    println!("Output: {:?}", output);
}
