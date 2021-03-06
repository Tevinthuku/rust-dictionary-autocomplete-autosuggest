use std::collections::HashMap;

static ENDS_HERE: &char = &'*';

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
        word_with_suffix.push(*ENDS_HERE);
        self.insert_internal(&word_with_suffix)
    }

    fn insert_internal(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }
        self.children
            .entry(word.chars().next().unwrap())
            .or_insert_with(Trie::new)
            .insert_internal(&word[1..])
    }

    pub fn find_words_based_on_prefix(&mut self, prefix: String) -> Option<Vec<String>> {
        let suffixes = self.get_suffixes_of_prefix(&prefix)?;
        let result = suffixes
            .into_iter()
            .map(|suffix| format!("{}{}", prefix, suffix))
            .collect();
        Some(result)
    }
    fn get_suffixes_of_prefix(&mut self, prefix: &str) -> Option<Vec<String>> {
        if prefix.is_empty() {
            return Some(self.get_list_of_remaining_words());
        }
        let child = self.children.get_mut(&prefix.chars().next().unwrap())?;
        let result = child.get_suffixes_of_prefix(&prefix[1..])?;
        Some(result)
    }

    fn get_list_of_remaining_words(&mut self) -> Vec<String> {
        let mut result = Vec::new();

        for (key, trie) in &mut self.children {
            let mut sub_results = Vec::new();

            if key == ENDS_HERE {
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
        let word_constructed = String::default();
        self.auto_suggest_internal(&wrong_word, word_constructed)
    }

    fn auto_suggest_internal(
        &mut self,
        wrong_word: &str,
        mut word_constructed_so_far: String,
    ) -> Option<Vec<String>> {
        if wrong_word.is_empty() {
            return Some(self.combine_word_with_available_suffixes(word_constructed_so_far));
        }
        let current_character = &wrong_word.chars().next().unwrap();
        let child = self.children.get_mut(current_character);
        match child {
            Some(trie) => {
                word_constructed_so_far.push(*current_character);
                trie.auto_suggest_internal(&wrong_word[1..], word_constructed_so_far)
            }
            None if word_constructed_so_far.is_empty() => None,
            None => {
                let result = self.combine_word_with_available_suffixes(word_constructed_so_far);
                Some(result)
            }
        }
    }

    fn combine_word_with_available_suffixes(&mut self, word_so_far: String) -> Vec<String> {
        let available_suffixes_of_word_so_far = self.get_list_of_remaining_words();
        available_suffixes_of_word_so_far
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
