use std::collections::HashMap;

static ENDS_HERE: &str = "*";

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

    fn fetch(&mut self, prefix: String) -> Vec<String> {
        let chars: Vec<char> = prefix.chars().collect();
        self.fetch_internal(chars)
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
        let mut result = Vec::new();

        for (key, trie) in &mut self.children {
            let mut sub_results = Vec::new();

            if key.to_string() == ENDS_HERE {
                sub_results.push(String::from(""))
            } else {
                sub_results = trie
                    .get_elements()
                    .into_iter()
                    .map(|st| {
                        let mut resulty = key.to_string();
                        resulty.push_str(&st);
                        resulty
                    })
                    .collect()
            }
            result.append(&mut sub_results)
        }
        result
    }
}

fn main() {
    let mut tr = Trie::new();
    tr.insert("Hi there".to_string());
    let answer = tr.fetch("Hi".to_string());
    println!("{:?}", answer)
}
