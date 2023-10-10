use rustc_hash::FxHashMap;
use std::cell::{Ref, RefCell};
use std::fs::read_to_string;
use std::io;
use std::ops::{Deref, DerefMut};
use std::time::SystemTime;

fn main() {
    let words: Vec<String> = read_to_string("dictionary.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let trie = generate_trie(words);
    loop {
        println!("enter a substring to find completions for: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");

        input.pop();
        if input == "stop" {
            break;
        }
        println!("input is: {}", input);
        let trie = trie.clone();
        let start = SystemTime::now();
        let completions = autocomplete(input, trie);
        let elapsed = start.elapsed().unwrap();
        match completions {
            Some(comps) => {
                println!("completions:\n {:?}", comps)
            }
            None => {
                println!("No completions found");
            }
        }
        println!("got completions in {elapsed:?}");
    }
}

#[derive(Default, Debug, Clone)]
struct TrieNode {
    is_end_of_word: bool,
    children: FxHashMap<char, RefCell<TrieNode>>,
}

#[derive(Default, Debug, Clone)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default().get_mut();
        }
        current_node.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = &node.take(),
                None => return false,
            }
        }

        current_node.is_end_of_word
    }
}

fn generate_trie(words: Vec<String>) -> Trie {
    let mut trie = Trie::new();
    for word in words {
        trie.insert(word.as_str());
    }
    trie
}

fn autocomplete(substr: String, trie: Trie) -> Option<Vec<String>> {
    let mut current_node = &trie.root;
    for char in substr.chars() {
        let existing = current_node.children.get(&char);
        if existing.is_none() {
            return None;
        } else {
            current_node = &existing.unwrap().into_inner();
        }
    }
    Some(get_words_from_node(current_node, substr))
}

fn get_words_from_node(node: &TrieNode, start: String) -> Vec<String> {
    let mut words = vec![];
    if node.is_end_of_word {
        words.push(start.clone());
    }
    for (char, node) in node.children.iter() {
        let new_word = format!("{}{}", start, char);
        let child_words = get_words_from_node(&node.take(), new_word);
        for word in child_words.into_iter() {
            words.push(word);
        }
    }
    words
}
