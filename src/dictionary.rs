use std::collections::HashMap;

static ENDS_HERE: &str = "*";

#[derive(Debug)]
pub struct Trie {
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            children: HashMap::new(),
        }
    }
    pub fn insert(&mut self, word: String) {
        let mut word_with_suffix = word;
        word_with_suffix.push_str(ENDS_HERE);
        let chars: Vec<char> = word_with_suffix.chars().collect();
        self.insert_internal(chars)
    }

    fn insert_internal(&mut self, chars: Vec<char>) {
        if chars.is_empty() {
            return;
        }
        self.children
            .entry(chars[0])
            .or_insert_with(Trie::new)
            .insert_internal(chars[1..].to_vec())
    }

    pub fn find_words_based_on_prefix(&mut self, prefix: String) -> Option<Vec<String>> {
        let chars: Vec<char> = prefix.chars().collect();
        let result = self.find_words_based_on_prefix_internal(chars)?;
        let result = result
            .into_iter()
            .map(|suffix| format!("{}{}", prefix, suffix))
            .collect();
        Some(result)
    }
    fn find_words_based_on_prefix_internal(&mut self, prefix: Vec<char>) -> Option<Vec<String>> {
        if prefix.is_empty() {
            return Some(self.get_elements());
        }
        let child = self.children.get_mut(&prefix[0])?;
        let result = child.find_words_based_on_prefix_internal(prefix[1..].to_vec())?;
        Some(result)
    }

    fn get_elements(&mut self) -> Vec<String> {
        let mut result = Vec::new();

        for (key, trie) in &mut self.children {
            let mut sub_results = Vec::new();

            if key.to_string() == ENDS_HERE {
                sub_results.push(String::from(""))
            } else {
                sub_results = trie.combine_word_with_available_suffixes(key.to_string())
            }
            result.append(&mut sub_results)
        }
        result
    }

    pub fn auto_suggest(&mut self, wrong_word: String) -> Option<Vec<String>> {
        if wrong_word.is_empty() {
            return None;
        }
        let correct_substring_so_far = String::default();
        self.auto_suggest_internal(wrong_word.chars().collect(), correct_substring_so_far)
    }

    fn auto_suggest_internal(
        &mut self,
        word_chars: Vec<char>,
        mut word_so_far: String,
    ) -> Option<Vec<String>> {
        if word_chars.is_empty() {
            return Some(self.combine_word_with_available_suffixes(word_so_far));
        }
        let current_character = &word_chars[0];
        let child = self.children.get_mut(current_character);
        match child {
            Some(trie) => {
                word_so_far.push(*current_character);
                trie.auto_suggest_internal(word_chars[1..].to_vec(), word_so_far)
            }
            None if word_so_far.is_empty() => None,
            None => {
                let result = self.combine_word_with_available_suffixes(word_so_far);
                Some(result)
            }
        }
    }

    fn combine_word_with_available_suffixes(&mut self, word_so_far: String) -> Vec<String> {
        let suffuxes_of_incomplete_word = self.get_elements();
        suffuxes_of_incomplete_word
            .into_iter()
            .map(|suffix| format!("{}{}", word_so_far, suffix))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_be_able_to_return_full_word_based_on_prefix() {
        let mut trie = Trie::new();
        trie.insert("Dog".to_string());
        trie.insert("Dogecoin".to_string());
        let full_words_available = trie
            .find_words_based_on_prefix("Dog".to_string())
            .expect("should return a list from the trie");

        assert!(full_words_available.contains(&"Dog".to_string()));
        assert!(full_words_available.contains(&"Dogecoin".to_string()));
    }

    #[test]
    fn should_return_none_if_prefix_provided_isnt_available() {
        let mut trie = Trie::new();
        trie.insert("Dog".to_string());
        trie.insert("Dogecoin".to_string());
        let full_words_available = trie.find_words_based_on_prefix("Dogecoins".to_string());
        assert_eq!(full_words_available, None)
    }
    #[test]
    fn should_auto_suggest_if_word_provided_isnt_available() {
        let mut trie = Trie::new();
        trie.insert("Dog".to_string());
        trie.insert("Dogecoin".to_string());
        let full_words_available = trie
            .auto_suggest("Dogecoins".to_string())
            .expect("Suggestions to be made available");
        assert_eq!(vec!["Dogecoin".to_string()], full_words_available)
    }
    #[test]
    fn should_not_bring_suggestions_if_similar_word_doesnt_exist_in_dictionary() {
        let mut trie = Trie::new();
        trie.insert("Dog".to_string());
        trie.insert("Dogecoin".to_string());
        let full_words_available = trie.auto_suggest("Cat".to_string());
        assert_eq!(full_words_available, None)
    }

    #[test]
    fn should_return_suggestions_if_similar_words_exist_in_dictionary_if_word_isnt_complete() {
        let mut trie = Trie::new();
        trie.insert("Dog".to_string());
        trie.insert("Dogecoin".to_string());
        let full_words_available = trie
            .auto_suggest("Do".to_string())
            .expect("Dog & Dogecoin to be returned");
        assert!(full_words_available.contains(&"Dog".to_string()));
        assert!(full_words_available.contains(&"Dogecoin".to_string()))
    }
}
