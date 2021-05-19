use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie {
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            children: HashMap::new(),
        }
    }
    fn insert(&mut self, word: String) {
        let mut word_with_suffix = word;
        word_with_suffix.push('*');
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

    fn fetch(&mut self, prefix: String) {
        let chars: Vec<char> = prefix.chars().collect();
        self.fetch_internal(chars);
    }
    fn fetch_internal(&mut self, prefix: Vec<char>) -> Vec<String> {
        if prefix.is_empty() {
            return self.get_elements();
        }
        let child = self.children.get_mut(&prefix[0]);
        match child {
            Some(trie) => trie.fetch_internal(prefix[1..].to_vec()),
            None => self.get_elements(),
        }
    }

    fn get_elements(&mut self) -> Vec<String> {
        println!("{:?}", self);
        Vec::new()
    }
}

fn main() {
    let mut tr = Trie::new();
    tr.insert("Hi there".to_string());
    tr.fetch("Hi".to_string())
}
