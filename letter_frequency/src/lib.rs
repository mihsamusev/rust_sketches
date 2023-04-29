use std::collections::HashMap;

fn group_anagrams<'a>(words: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut map = HashMap::new();
    for &word in words {
        let key = get_letter_frequency(word).to_key();
        map.entry(key).or_insert(Vec::new()).push(word);
    }
    map.into_values().collect()
}

#[derive(Debug, Default, Eq, PartialEq)]
struct LetterFrequency([u32; 26]);

impl LetterFrequency {
    fn to_key(&self) -> String {
        let mut key = String::with_capacity(52);
        for i in self.0.iter() {
            key.push('#');
            key.push(char::from_u32(*i).unwrap());
        }
        key
    }
}
// Assumptions
// only ascii letters
// uppercase and lowercase are treated the same
//
fn get_letter_frequency(word: &str) -> LetterFrequency {
    let mut frequency = LetterFrequency::default();
    for c in word.chars() {
        if let Some(position) = (c.to_ascii_lowercase() as usize).checked_sub('a' as usize) {
            frequency.0[position] += 1;
        }
    }
    frequency
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_word_generate_correct_frequency_vector() {
        let mut zeros = [0_u32; 26];
        zeros[..3].copy_from_slice(&[1, 2, 3]);
        assert_eq!(get_letter_frequency("abbccc").0, zeros);
    }

    #[test]
    fn given_list_of_anagrams_produce_list_of_grouped_word_sets() {
        let words = vec!["word", "sword", "drow", "rowd", "iced", "dice"];
        let mut expected = vec![
            vec!["word", "drow", "rowd"],
            vec!["sword"],
            vec!["iced", "dice"],
        ];
        expected.sort();
        assert_eq!(group_anagrams(&words), expected);
    }
}
