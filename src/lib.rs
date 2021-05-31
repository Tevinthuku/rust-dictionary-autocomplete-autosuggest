#![crate_name = "auto_correct_n_suggest"]
mod dictionary;

/// This dictionary will hold all the words/keywords you want to use within your program.
pub struct Dictionary {
    internal: dictionary::Trie,
}

impl Default for Dictionary {
    /// Returns a new dictionary which will always be empty
    ///
    /// # Examples
    /// ```
    /// use auto_correct_n_suggest;
    /// let dictionary = auto_correct_n_suggest::Dictionary::default();
    ///
    /// ```
    fn default() -> Self {
        Dictionary::new()
    }
}

impl Dictionary {
    /// Returns a new dictionary just like the `default` method
    /// The dictionary will always be empty at first, but you can add
    /// words to it using the `insert` method
    ///
    /// # Examples
    ///
    /// ```
    /// use auto_correct_n_suggest;
    /// let mut dictionary = auto_correct_n_suggest::Dictionary::new();
    /// ```
    pub fn new() -> Dictionary {
        Dictionary {
            internal: dictionary::Trie::new(),
        }
    }

    /// Adds a new word to the dictionary
    /// Provide a Word `String` to an existing dictionary to add it the dictionary
    ///
    /// # Arguments
    ///
    /// * `word` - The string to be inserted to the dictionary
    ///
    /// # Examples
    /// ```
    /// use auto_correct_n_suggest;
    /// let mut dictionary = auto_correct_n_suggest::Dictionary::new();
    /// let new_coin = format!("{}", "Dogecoin");
    /// dictionary.insert(new_coin);
    ///
    /// ```
    pub fn insert(&mut self, word: String) {
        self.internal.insert(word)
    }

    /// This method helps you get the complete words available based on the prefix provided
    /// # Arguments
    ///
    /// * `prefix` - A string which represents the prefix of words in the dictionary
    ///
    /// It returns `None` if there are no suffixes of the prefix provided
    /// # Examples
    ///
    /// ```
    /// use auto_correct_n_suggest;
    /// let mut dictionary = auto_correct_n_suggest::Dictionary::new();
    /// let word1 = "Dog".to_string();
    /// dictionary.insert(word1);
    /// let words_available = dictionary
    ///     .find_words_based_on_prefix("Dog".to_string())
    ///     .expect("Words should be present");
    /// assert!(words_available.contains(&"Dog".to_string()));
    /// ```
    ///
    pub fn find_words_based_on_prefix(&mut self, prefix: String) -> Option<Vec<String>> {
        self.internal.find_words_based_on_prefix(prefix)
    }

    /// This method helps you get alternative words based in your dictionary that are similar to the provided word
    /// # Arguments
    ///
    /// * `typo` - The mistyped word
    ///
    /// It returns `None` if no word in the dictionary matches any sequence of characters of the word given.
    /// # Examples
    /// ```
    /// use auto_correct_n_suggest;
    /// let mut dictionary = auto_correct_n_suggest::Dictionary::new();
    /// let word1 = "Dog".to_string();
    /// let word2 = "Dogecoin".to_string();
    ///
    /// dictionary.insert(word1);
    /// dictionary.insert(word2);
    ///
    /// let words_available = dictionary
    ///     .auto_suggest_alternative_words("Dogecoins".to_string())
    ///     .expect("Suggestions should be available");
    ///
    /// assert_eq!(vec!["Dogecoin".to_string()], words_available);
    /// // In this dictionary there's Cats so there isnt any alternative word for Cat
    /// let words_available = dictionary.auto_suggest_alternative_words("Cat".to_string());
    /// assert_eq!(words_available, None);
    ///```
    ///
    ///
    pub fn auto_suggest_alternative_words(&mut self, typo: String) -> Option<Vec<String>> {
        self.internal.auto_suggest(typo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_provide_full_words_if_word_based_on_prefix() {
        let mut dictionary = Dictionary::new();
        let word1 = "Dog".to_string();
        let word2 = "Dogecoin".to_string();
        dictionary.insert(word1);
        dictionary.insert(word2);
        let words_available = dictionary
            .find_words_based_on_prefix("Dog".to_string())
            .expect("Words should be present");

        assert!(words_available.contains(&"Dog".to_string()));
        assert!(words_available.contains(&"Dogecoin".to_string()))
    }
    #[test]
    fn should_auto_correct_if_word_provided_isnt_available() {
        let mut dictionary = Dictionary::new();
        let word1 = "Dog".to_string();
        let word2 = "Dogecoin".to_string();
        dictionary.insert(word1);
        dictionary.insert(word2);
        let words_available = dictionary
            .auto_suggest_alternative_words("Dogecoins".to_string())
            .expect("Suggestions should be available");
        assert_eq!(vec!["Dogecoin".to_string()], words_available)
    }

    #[test]
    fn should_return_none_if_word_provided_isnt_available_and_there_are_no_matching_words() {
        let mut dictionary = Dictionary::new();
        let word1 = "Dog".to_string();
        let word2 = "Dogecoin".to_string();
        dictionary.insert(word1);
        dictionary.insert(word2);
        let words_available = dictionary.auto_suggest_alternative_words("Cat".to_string());
        assert_eq!(words_available, None)
    }
}
