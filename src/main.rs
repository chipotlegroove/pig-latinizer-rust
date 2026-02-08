use text_io::read;

fn main() {
    let words: String = read!("{}\n");

    let split_words = words.split_whitespace();
    let mut entencesay = vec![];
    for word in split_words {
        let ordway = convert_to_pig_latin(word);
        entencesay.push(ordway);
    }
    let joined = entencesay.join(" ");

    println!("{joined}");
}

fn convert_to_pig_latin(word: &str) -> String {
    let first_vowel_position = find_first_vowel_position(word);

    let translation = match first_vowel_position {
        Some(position) => {
            if position == 0 {
                translate_simple_word(&word, true)
            } else {
                translate_consonant_starting_word(&word, position)
            }
        }
        None => translate_simple_word(&word, false),
    };

    translation
}

fn translate_simple_word(word: &str, has_vowels: bool) -> String {
    let chars = word.chars();
    let mut punctuations = vec![];
    let mut word = vec![];

    for char in chars {
        if char.is_ascii_punctuation() {
            punctuations.push(char.to_ascii_lowercase());
        } else {
            word.push(char.to_ascii_lowercase());
        }
    }

    let punctuations: String = punctuations.into_iter().collect();
    let word: String = word.into_iter().collect();

    let suffix = if has_vowels {
        String::from("way")
    } else {
        String::from("ay")
    };

    format!("{word}{suffix}{punctuations}")
}

fn translate_consonant_starting_word(word: &str, first_vowel_position: usize) -> String {
    let chars = word.chars().enumerate();
    let mut first_consonants = vec![];
    let mut punctuations = vec![];
    let mut rest_of_word = vec![];

    for (i, char) in chars {
        if char.is_ascii_punctuation() {
            punctuations.push(char);
            continue;
        }
        if i < first_vowel_position {
            first_consonants.push(char.to_ascii_lowercase());
        } else {
            rest_of_word.push(char.to_ascii_lowercase());
        }
    }

    let first_consonants: String = first_consonants.iter().collect();
    let rest_of_word: String = rest_of_word.iter().collect();
    let punctuations: String = punctuations.iter().collect();
    format!("{rest_of_word}{first_consonants}ay{punctuations}")
}

fn find_first_vowel_position(word: &str) -> Option<usize> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    word.chars()
        .position(|c| vowels.contains(&c.to_ascii_lowercase()))
}
