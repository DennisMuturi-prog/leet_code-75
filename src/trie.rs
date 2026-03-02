use std::collections::HashMap;
#[derive(Default)]
pub struct Trie {
    is_end: bool,
    children: HashMap<char, Trie>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn insert(&mut self, word: String) {
        let mut current_node = self;
        for letter in word.chars() {
            current_node = current_node.children.entry(letter).or_default();
        }
        current_node.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current_node = self;
        for letter in word.chars() {
            match current_node.children.get(&letter) {
                Some(node) => {
                    current_node = node;
                }
                None => return false,
            }
        }
        current_node.is_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut current_node = self;
        for letter in prefix.chars() {
            match current_node.children.get(&letter) {
                Some(node) => {
                    current_node = node;
                }
                None => return false,
            }
        }
        true
    }
    pub fn delete(&mut self, word: String) {
        if !self.search(word.clone()) {
            return;
        }
        let node = self;
        let word: Vec<char> = word.chars().collect();
        let current_letter_pos = 0;
        Trie::delete_word(node, current_letter_pos, &word);
    }
    fn delete_word(node: &mut Trie, current_letter_pos: usize, word: &[char]) -> bool {
        if current_letter_pos == word.len() {
            node.is_end = false;
            return node.children.is_empty();
        }

        let next_child = match node.children.get_mut(&word[current_letter_pos]) {
            Some(val) => val,
            None => return false,
        };

        let next_deletion = Trie::delete_word(next_child, current_letter_pos + 1, word);
        if next_deletion {
            node.children.remove(&word[current_letter_pos]);
        }
        next_deletion && !node.is_end && node.children.is_empty()
    }
    pub fn words_starting_with(&self, word: String) -> Vec<String> {
        let base = word.clone();
        let mut possible_words = Vec::new();
        let mut current_node = self;
        for letter in word.chars() {
            match current_node.children.get(&letter) {
                Some(node) => {
                    current_node = node;
                }
                None => return possible_words,
            }
        }
        if current_node.is_end {
            possible_words.push(base.clone());
        }
        let mut base: Vec<char> = base.chars().collect();
        for (key, node) in current_node.children.iter() {
            Trie::dfs(&mut base, node, &mut possible_words, *key);
        }

        possible_words
    }
    fn dfs(word: &mut Vec<char>, node: &Trie, possible_words: &mut Vec<String>, key: char) {
        word.push(key);
        if node.children.is_empty() || node.is_end {
            possible_words.push(word.iter().collect());
        }
        for (new_key, new_node) in node.children.iter() {
            Trie::dfs(word, new_node, possible_words, *new_key);
        }
        word.pop();
    }
}
