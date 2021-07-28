use std::{env, io};
use lib::LanguageModel;
pub mod lib;

fn main() {
    let lm = LanguageModel::new();
    let mut input = String::new();
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Couldn't process input");
        for word in lm.untangle(&input) {
            print!("{} ", word);
        }
    } else if args.len() > 1 {
        for word_pile in &args[1..] {
            for word in lm.untangle(word_pile) {
                print!("{} ", word);
            }
        }
    }
}
