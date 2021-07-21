use std::{env, io};
use lib::LanguageModel;
pub mod lib;

fn main() {
    let dict_path = include_str!("dicts/english.txt");
    let lm = LanguageModel::new(dict_path);
    let mut input = String::new();
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Couldn't process input");
        for word in lm.split(input) {
            print!("{} ", word);
        }
    } else if args.len() > 1 {
        for word_pile in &args[1..] {
            for word in lm.split(String::from(word_pile)) {
                print!("{} ", word);
            }
        }
    }
}
