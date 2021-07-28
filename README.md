![Untanglr](.github/banner.jpg)
# Untanglr

Untanglr takes in a some mangled words and makes sense out of them so you dont have to. It goes through the input and splits it probabilistically into words. The crate includes both a bin.rs and a lib.rs to facilitate both usage as a command line utility, and as a library that you can use in your code.

# Usage
Pass the tangled words as a cli argument:
```bash
$ untanglr thequickbrownfoxjumpedoverthelazydog
the quick brown fox jumped over the lazy dog
```

Or use it in your projects:
```rust
extern crate untanglr;

fn main() {
	let lm = untanglr::LanguageModel::new();
	println!("{:?}", lm.untangle(String::from("helloworld")));
}
```

# Installation
If you find that untanglr might be useful on your machine you can install it. Just make sure cargo is installed and run:
```
$ cargo install untanglr
```
**Note:** Don't be discouraged if this project hasn't been updated in a while. I will address potential issues but the crate does not need regular updates.

# Credits
I have developed this project around Derek Anderson's [wordninja](https://github.com/keredson/wordninja) python implementation for some exercising in rust while producing something useful.

