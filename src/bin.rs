use std::io;
pub mod lib;
use lib::LanguageModel;
fn main() {
    let lm = LanguageModel::new();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Couldn't read line");

    for word in lm.split(input) {
        print!("{} ", word);
    }
}
