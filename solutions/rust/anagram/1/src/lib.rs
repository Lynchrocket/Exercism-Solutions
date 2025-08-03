use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut word_chars = word.chars().collect::<Vec<char>>();
    word_chars.sort();
    possible_anagrams
        .iter()
        .filter(|anagram| {
            let anagram = anagram.to_lowercase();
            if anagram.to_lowercase() == word {
                return false;
            } else {
                let mut anagram_chars = anagram.chars().collect::<Vec<char>>();
                anagram_chars.sort();
                word_chars.eq(&anagram_chars)
            }
        })
        .cloned()
        .collect()
}
