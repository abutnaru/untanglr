# Untanglr
**Mention:** Don't be discuraged if this project hasn't been updated in a while. I will address potential issues but the crate does not need regular updates.
Untanglr takes in a some mangled words and makes sense out of them so you dont have to. It goes through the input and splits it probabilistically into words.

# Usage
Pass the tangled words as a cli argument:
```
$ untanglr thequickbrownfoxjumpedoverthelazydog
the quick brown fox jumped over the lazy dog
```

Or use it in your projects:
```
extern crate untanglr;

fn main() {
	let lm = untanglr::LanguageModel::new();
	println!("{:?}", lm.split(String::from("helloworld:)));
}
```

# Installation
If you find that untanglr might be useful on your machine you can install it. Just make sure cargo is installed and run:
```
cargo install untanglr
```

# Credits
I have developed this project around Derek Anderson's [wordninja](https://github.com/keredson/wordninja) python implementation for some exercising in rust while producing something useful.

