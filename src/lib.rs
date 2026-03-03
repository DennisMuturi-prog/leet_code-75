pub mod trie;
pub mod union_find;
#[cfg(test)]
mod tests {
    use crate::trie::Trie;

    #[test]
    fn trie_inserts() {
        let mut trie = Trie::default();
        trie.insert("car".to_string());
        trie.insert("carpet".to_string());
        assert!(trie.search("car".to_string()));
        assert!(trie.search("carpet".to_string()));
        assert!(trie.starts_with("ca".to_string()));
        assert!(trie.starts_with("carp".to_string()));
        assert!(!trie.search("carp".to_string()));
    }
    #[test]
    fn trie_deletes() {
        let mut trie = Trie::default();
        trie.insert("car".to_string());
        trie.insert("carpet".to_string());
        assert!(trie.search("car".to_string()));
        assert!(trie.search("carpet".to_string()));
        assert!(trie.starts_with("ca".to_string()));
        assert!(trie.starts_with("carp".to_string()));
        assert!(!trie.search("carp".to_string()));
        trie.delete("car".to_string());
        assert!(!trie.search("car".to_string()));
        assert!(trie.search("carpet".to_string()));
        trie.delete("carpet".to_string());
        assert!(!trie.search("carpet".to_string()));
        assert!(!trie.starts_with("ca".to_string()));
    }
    #[test]
    pub fn trie_retrieves_with_certain_prefix() {
        let mut trie = Trie::default();
        trie.insert("car".to_string());
        trie.insert("carpet".to_string());
        trie.insert("crab".to_string());
        trie.insert("crime".to_string());
        trie.insert("trapeze".to_string());
        trie.insert("trap".to_string());
        trie.insert("type".to_string());
        trie.insert("typed".to_string());
        trie.insert("typo".to_string());
        let mut possible_words = trie.words_starting_with("c".to_string());
        possible_words.sort();

        assert_eq!(
            vec![
                "car".to_string(),
                "carpet".to_string(),
                "crab".to_string(),
                "crime".to_string(),
            ],
            possible_words
        );
        let mut possible_words = trie.words_starting_with("car".to_string());
        possible_words.sort();
        assert_eq!(
            vec!["car".to_string(), "carpet".to_string()],
            possible_words
        );
        let mut possible_words = trie.words_starting_with("carp".to_string());
        possible_words.sort();
        assert_eq!(vec!["carpet".to_string()], possible_words);
        let mut possible_words = trie.words_starting_with("cr".to_string());
        possible_words.sort();
        assert_eq!(
            vec!["crab".to_string(), "crime".to_string()],
            possible_words
        );
        let mut possible_words = trie.words_starting_with("trap".to_string());
        possible_words.sort();
        assert_eq!(
            vec!["trap".to_string(), "trapeze".to_string()],
            possible_words
        );
        let mut possible_words = trie.words_starting_with("typ".to_string());
        possible_words.sort();
        assert_eq!(
            vec!["type".to_string(), "typed".to_string(), "typo".to_string()],
            possible_words
        );
        let mut possible_words = trie.words_starting_with("type".to_string());
        possible_words.sort();
        assert_eq!(
            vec!["type".to_string(), "typed".to_string()],
            possible_words
        );
        assert_eq!(
            Vec::<String>::new(),
            trie.words_starting_with("vui".to_string())
        );
    }
}
