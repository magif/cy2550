use clap::Parser;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 4)]
    words: i32,

    #[clap(short, long, default_value_t = 0)]
    caps: i32,

    #[clap(short, long, default_value_t = 0)]
    numbers: i32,

    #[clap(short, long, default_value_t = 0)]
    symbols: i32,
}

fn main() {
    let args = Args::parse();

    let filename = "words.txt";
    let contents = fs::read_to_string(filename).expect("Failed to read \"words.txt\"");
    let words: Vec<&str> = contents.split_whitespace().collect();
    let rand_words: Vec<String> = random_words(&words, &args.words);
    let num_caps: usize = std::cmp::min(args.caps, args.words) as usize;
    let cap_words: Vec<String> = cap_random_words(&rand_words, num_caps);
    let num_nums: usize = args.numbers as usize;
    let num_words: Vec<String> = insert_nums(&cap_words, num_nums);
    let num_syms: usize = args.symbols as usize;
    let final_words: Vec<String> = insert_symbols(&num_words, num_syms);
    let password: String = final_words.join("");
    println!("{}", password);
}

fn random_words(words: &Vec<&str>, count: &i32) -> Vec<String> {
    let selection = words.choose_multiple(&mut rand::thread_rng(), *count as usize);
    let selection_strs = selection.map(|s| *s);
    let selection_vec: Vec<&str> = selection_strs.collect();
    let selection_vec_string: Vec<String> =
        selection_vec.into_iter().map(|s| String::from(s)).collect();
    selection_vec_string
}

fn capitalize_word(word: &String) -> String {
    let chars: Vec<char> = word.chars().collect();
    let cap_letter: char = chars[0].to_uppercase().nth(0).unwrap();
    let cap_letter_vec: Vec<char> = vec![cap_letter];
    let remain_letters: Vec<char> = chars[1..].to_vec();
    let new_word_vec = [cap_letter_vec, remain_letters].concat();
    let new_word: String = new_word_vec.into_iter().collect();
    new_word
}

fn cap_random_words(words: &Vec<String>, count: usize) -> Vec<String> {
    let cap_words: Vec<String> = words[..count]
        .into_iter()
        .map(|s| capitalize_word(s))
        .collect();
    let lower_words: Vec<String> = words[count..].to_vec();
    let mut all_words: Vec<String> = [cap_words, lower_words].concat();
    all_words.shuffle(&mut rand::thread_rng());
    all_words
}

fn insert_item(words: &Vec<String>, item: String) -> Vec<String> {
    let rand_index: usize = rand::thread_rng().gen_range(0..words.len() + 1);
    let before_vec: Vec<String> = words[..rand_index].to_vec();
    let after_vec: Vec<String> = words[rand_index..].to_vec();
    let middle_vec: Vec<String> = vec![item];
    let inserted_vec: Vec<String> = [before_vec, middle_vec, after_vec].concat();
    inserted_vec
}

fn insert_num(words: &Vec<String>) -> Vec<String> {
    let rand_num: u8 = rand::thread_rng().gen_range(0..10);
    let rand_num_as_str: String = rand_num.to_string();
    let inserted_vec: Vec<String> = insert_item(&words, rand_num_as_str);
    inserted_vec
}

fn insert_nums(words: &Vec<String>, count: usize) -> Vec<String> {
    let mut curr_words = words.clone();
    for _ in 0..count {
        curr_words = insert_num(&curr_words);
    }
    curr_words
}

fn insert_symbol(words: &Vec<String>) -> Vec<String> {
    let symbols: Vec<String> = "~!@#$%^&*.:;"
        .split("")
        .into_iter()
        .map(|s| String::from(s))
        .collect();
    let rand_symbol: String = symbols
        .choose(&mut rand::thread_rng())
        .unwrap_or(&String::from("~"))
        .to_string();
    let inserted_vec: Vec<String> = insert_item(&words, rand_symbol);
    inserted_vec
}

fn insert_symbols(words: &Vec<String>, count: usize) -> Vec<String> {
    let mut curr_words = words.clone();
    for _ in 0..count {
        curr_words = insert_symbol(&curr_words);
    }
    curr_words
}
