mod dictionary;

pub struct Dictionary {
    internal: dictionary::Trie,
}

impl Default for Dictionary {
    fn default() -> Self {
        Dictionary::new()
    }
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            internal: dictionary::Trie::new(),
        }
    }
    pub fn insert(&mut self, word: String) {
        self.internal.insert(word)
    }

    pub fn find_words_based_on_prefix(&mut self, prefix: String) -> Option<Vec<String>> {
        self.internal.find_words_based_on_prefix(prefix)
    }

    pub fn auto_suggest_alternative_words(&mut self, word: String) -> Option<Vec<String>> {
        self.internal.auto_suggest(word)
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
