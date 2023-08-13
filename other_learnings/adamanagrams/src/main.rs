use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::io;
use std::time::SystemTime;

const LETTERS: [(char, usize); 26] = [
    ('a', 219694),
    ('b', 59124),
    ('c', 128689),
    ('d', 100204),
    ('e', 252898),
    ('f', 35210),
    ('g', 75010),
    ('h', 82819),
    ('i', 225925),
    ('j', 5386),
    ('k', 25362),
    ('l', 157609),
    ('m', 93615),
    ('n', 188615),
    ('o', 185267),
    ('p', 98743),
    ('q', 5769),
    ('r', 198777),
    ('s', 184724),
    ('t', 182889),
    ('u', 114387),
    ('v', 31737),
    ('w', 21414),
    ('x', 21414),
    ('y', 66545),
    ('z', 13977),
];

lazy_static! {
    static ref DICT: String = read_to_string("dictionary.txt").unwrap();
    static ref INDICIES: Vec<HashSet<String>> = {
        let mut vec = Vec::with_capacity(26);
        for letter in LETTERS {
            vec.push(set_index(letter));
        }
        vec
    };
}

fn main() {
    let _d = DICT.len();
    let _i = INDICIES.len();
    println!("enter 'stop' to exit");
    loop {
        println!("enter a word to find anagrams for: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");

        input.pop();
        if input == "stop" {
            break;
        }
        println!("input is: {}", input);
        let start = SystemTime::now();
        let anagrams = find_anagrams(input);
        let elapsed = start.elapsed().unwrap();
        println!("anagrams are: ");
        for word in anagrams.iter() {
            println!("{word}");
        }
        println!("number of anagrams: {}", anagrams.len());
        println!("got anagrams in {elapsed:?}");
    }
}

fn set_index(c: (char, usize)) -> HashSet<String> {
    let mut set = HashSet::with_capacity(c.1);
    for word in DICT.split("\n") {
        if word.chars().any(|letter| letter == c.0) {
            set.insert(word.to_string());
        }
    }
    set
}

fn get_index_for(c: char) -> HashSet<String> {
    let index = LETTERS
        .iter()
        .position(|&x| x.0 == c)
        .expect(format!("{} not found in LETTERS", c).as_str());
    return INDICIES.get(index).unwrap().clone();
}

fn find_anagrams(s: String) -> HashSet<String> {
    let mut vec = Vec::with_capacity(s.len());
    let mut indicies: Vec<HashSet<String>> = s
        .chars()
        .into_iter()
        .filter_map(|c| {
            if !vec.contains(&c) {
                vec.push(c);
                Some(get_index_for(c))
            } else {
                None
            }
        })
        .collect::<Vec<HashSet<String>>>();
    for set in &mut indicies {
        set.retain(|word| {
            for c in word.chars() {
                if !s.contains(c) || word.matches(c).count() > s.matches(c).count() {
                    return false;
                }
            }
            return true;
        })
    }

    indicies
        .iter()
        .fold(HashSet::with_capacity(370105), |acc, set| {
            acc.union(set).cloned().collect()
        })
}
