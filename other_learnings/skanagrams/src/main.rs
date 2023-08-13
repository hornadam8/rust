use std::env::args;
use std::fs::read_to_string;
use std::time::SystemTime;

fn main() {
    let words: Vec<String> = read_to_string("dictionary.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let args: Vec<String> = args().collect();
    let binding = "".to_string();
    let iword: &String = args.get(1).unwrap_or(&binding);
    // Convert to lowercase
    let iword_lc: &String = &iword.to_lowercase();

    // Validate iword
    if !is_all_lowercase_letters(iword_lc) {
        println!(
            "Error: Invalid input. The word should only contain lowercase letters from 'a' to 'z'."
        );
        return;
    }

    let start = SystemTime::now();

    let (anagrams, count) = anagrams(iword_lc, words);
    let elapsed = start.elapsed().unwrap();
    println!(
        "made {} anagrams for word {} in {:?}",
        count, iword, elapsed
    );
    println!("anagrams \n {:?}", anagrams)
}
fn is_all_lowercase_letters(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn anagrams(iword: &String, words: Vec<String>) -> (Vec<String>, u32) {
    let mut anagrams: Vec<String> = Vec::new();
    let mut count: u32 = 0;
    for word in words.iter() {
        // this is kinda jacked
        if is_anagram(&iword, &&word.as_str()) {
            anagrams.push(word.to_string());
            count += 1;
        }
    }
    (anagrams, count)
}

fn is_anagram(word1: &String, word2: &&&str) -> bool {
    // this need to be modified to generate words in a sorted chars counting up for subwords and return a vec
    let mut chars1: Vec<char> = word1.chars().collect();
    let mut chars2: Vec<char> = word2.chars().collect();
    chars1.sort();
    chars2.sort();
    chars1 == chars2
}
