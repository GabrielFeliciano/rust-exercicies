use std::collections::{HashSet};

mod anagram {
    use std::collections::{HashMap};

    type CharCount = HashMap<char, u8>;

    pub struct PossibleAnagram<'a> {
        pub word: &'a str,
        pub char_count: CharCount,
        _secret: ()
    }

    impl<'a> PossibleAnagram<'a> {
        pub fn new(word: &'a str) -> Self {
            let mut char_count: CharCount = HashMap::new();
            let word_lowercase = word.to_lowercase();

            for letter in word_lowercase.chars() {
                let entry = char_count.entry(letter).or_insert(0);
                *entry += 1;
            }

            PossibleAnagram {
                word,
                char_count,
                _secret: ()
            }
        }

        pub fn is_anagram_from (&self, other: &Self) -> bool {
            self.char_count == other.char_count 
            && 
            self.word.to_lowercase() != other.word.to_lowercase()
        }
    }
}

use crate::anagram::PossibleAnagram;

pub fn anagrams_for<'a, 'b>(word: &str, possible_anagrams: &'b[& str]) -> HashSet<&'b str> {
    let mut anagrams_set: HashSet<&'b str> = HashSet::new();

    let chosen_word_possible_anagram = PossibleAnagram::new(word);
    let possible_anagrams_mapped = possible_anagrams
        .iter()
        .map(|a_word| PossibleAnagram::new(a_word));

    for other_possible_anagram in possible_anagrams_mapped {
        if chosen_word_possible_anagram.is_anagram_from(&other_possible_anagram) {
            anagrams_set.insert(other_possible_anagram.word);
        }
    }

    anagrams_set
}
