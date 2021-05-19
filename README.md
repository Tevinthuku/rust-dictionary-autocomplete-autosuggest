## auto_correct_n_suggest

### Description

A rust Dictionary Trie. Performs autosuggestions on words with typos & autocompletes words 🦀

### How to use

```rs
let mut dictionary = Dictionary::new();
let word1 = "Dog".to_string();
let word2 = "Dogecoin".to_string();
dictionary.insert(word1);
dictionary.insert(word2);

let words_available = dictionary.find_words_based_on_prefix("Dog".to_string())?; // vec!["Dog", "Dogecoin"]

let typo_auto_suggestions = dictionary.auto_correct("Dogecoins".to_string())?; // vec!["Dogecoin"]

```

TODO:

1. Add support for case in-sensitivity
