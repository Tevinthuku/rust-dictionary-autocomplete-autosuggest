## auto_correct_n_suggest

### Description

A rust Dictionary Trie. Performs autosuggestions on words with typos & autocompletes words 🦀

[Link to crate](https://crates.io/crates/auto_correct_n_suggest)

## Installation

Add

```
auto_correct_n_suggest = "0.1.0"
```

to your Cargo.toml

### How to use

```rust
use auto_correct_n_suggest;

let mut dictionary = auto_correct_n_suggest::Dictionary::new();
let word1 = "Dog".to_string();
let word2 = "Dogecoin".to_string();
dictionary.insert(word1);
dictionary.insert(word2);

let words_available = dictionary.find_words_based_on_prefix("Dog".to_string())?; // vec!["Dog", "Dogecoin"]

let typo_auto_suggestions = dictionary.auto_correct("Dogecoins".to_string())?; // vec!["Dogecoin"]

```
