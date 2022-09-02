use std::{env, io};
use lib::LanguageModel;
pub mod lib;

fn main() {
    let args: Vec<_> = env::args().collect();
    let lm = LanguageModel::new();
    let mut input = String::new();
    let mut sentence = String::new();

    if args.len() == 1 {
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Couldn't process input");
        for word in lm.untangle(&input) {
            sentence.push_str(&format!("{word} "));
        }
        print!("{}", sentence.trim_end());
    } else if args.len() > 1 {
        for word_pile in &args[1..] {
            for word in lm.untangle(word_pile) {
                sentence.push_str(&format!("{word} "))
            }
            print!("{}\n", sentence.trim_end());
            sentence.clear();
        }
    }
}
