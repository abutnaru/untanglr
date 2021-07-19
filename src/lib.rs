extern crate regex;
use regex::bytes::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str;

// The LanguageModel holds the path to the dictionary (english
// by default), the maximum word cost and the maximum word length
pub struct LanguageModel {
    //TODO: Add support for custom dictionaries
    //dict: String,
    word_cost: HashMap<String, f64>,
    max_wlen: u8,
}

impl LanguageModel {
    // Initialize the LanguageModel using the words in the dictionary to
    // get the maximum word length and calculate word cost uzing Zipf's law
    pub fn new() -> LanguageModel {
        //TODO: This should be able to take user's input if needed
        let path = Path::new("dicts/english.txt");
        let file = match File::open(&path) {
            Err(e) => panic!("Couldn't open {}: {}", path.display(), e),
            Ok(file) => file,
        };
        let buffered = BufReader::new(file);
        let reader: Vec<_> = buffered.lines().collect();
        let word_count: f64 = reader.len() as f64;
        let mut max_wlen: u8 = 0;
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        let mut word_cost: HashMap<String, f64> = HashMap::new();
        for (index, line) in reader.into_iter().enumerate() {
            let word = line.unwrap(); // Ignore errors.
            let word_len: u8 = word.chars().count() as u8;
            max_wlen = max(word_len, max_wlen);

            word_cost.insert(word, ((index + 1) as f64 * word_count.ln()).ln());
        }
        //println!("{:#?}",word_cost);
        LanguageModel {
            word_cost,
            max_wlen,
        }
    }
    pub fn split(&self, s: String) -> Vec<String> {
        //let mut result = vec![];
        let re = Regex::new(r"[^a-zA-Z0-9']+").unwrap();
        let mut result: Vec<String> = Vec::new();

        for c in re.split(s.as_bytes()) {
            let capture = match str::from_utf8(c) {
                Ok(s) => s,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            //println!("{}", capture);
            for s in self.slice(String::from(capture)) {
                &result.push(s);
            }
        }
        result
    }

    fn slice(&self, s: String) -> Vec<String> {
        let mut cost: Vec<f64> = vec![0.0];
        for i in 1..s.len() + 1 {
            //println!("{:?}",i);
            let (c, k) = self.best_match(i, &cost, &s);
            //println!("best_match{:?}", self.best_match(i,&cost,&s));
            //println!("cost:{:?}",cost);

            cost.push(k);
        }

        let mut out: Vec<String> = Vec::new();
        let mut i = s.len();
        while i > 0 {
            let (k, c) = self.best_match(i, &cost, &s);
            //println!("{:?} == {:?}", cost[i], c);
            assert!(c == cost[i]);

            let mut new_token = true;
            let z: usize = 0;
            let outlen = out.len();

            let idx = if (i - k as usize) < 0 {
                0
            } else {
                i - k as usize
            };
            if &s[idx..i] != "'" {
                if &out.len() > &z {
                    let o = &out[out.len() - 1];
                    if out[out.len() - 1] == "'s"
                        || (s.as_bytes()[i - 1].is_ascii_digit()
                            && o.as_bytes()[0].is_ascii_digit())
                    {
                        out[outlen - 1] =
                            format!("{}{}", &s[i - k as usize..i], &out[out.len() - 1]);
                        new_token = false;
                    }
                }
            }
            if new_token {
                //let idx2 = if (i as f64 - k) < 0.0 {0} else {i - k as usize};
                //println!("s: {:?}",s);
                //println!("selection: {:?}, {:?}", i-k as usize, i);
                out.push(s[i - k as usize..i].to_string());
            }

            //let idx3 = if (i as f64 - k) < 0.0 {0} else {i - k  as usize};
            i -= k as usize;
        }
        out.reverse();
        //println!("out.rev: {:?}", out);
        out
    }

    fn best_match(&self, i: usize, cost: &[f64], s: &String) -> (u8, f64) {
        let mut candidates = &cost[(max(0, i as i64 - self.max_wlen as i64) as usize)..i];
        let mut storedmin: (u8, f64) = (0, 9e99);

        for (index, &candidate) in candidates.into_iter().rev().enumerate() {
            let current_word_cost = match self.word_cost.get(&s[i - index - 1..i]) {
                Some(value) => value,
                _ => &(9e99 as f64),
            };
            //println!("CURRENT WORD: {:?}",&s[i - index - 1..i]);
            //
            //println!("i:{:?}, index:{:?}, wc:{:?}",i,index, current_word_cost);
            if candidate + current_word_cost < storedmin.1 {
                storedmin.1 = candidate + *current_word_cost;
                storedmin.0 = index as u8 + 1;
            }
        }
        storedmin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_split() {
        let no_spaces = "thequickbrownfoxjumpedoverthelazydog".to_string();
        let with_spaces = "the quick brown fox jumped over the lazy dog";
        let lm = LanguageModel::new();
        let correct: Vec<&str> = with_spaces.split_whitespace().collect();
        assert_eq!(lm.split(no_spaces), correct);
    }
}
