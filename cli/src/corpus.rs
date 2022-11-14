use algae_lib::NgramData;
use serde::{Deserialize, Serialize};
use std::fs::File;

pub fn get_corpus<const N: usize>(source: &str, data: &str) -> NgramData<N>
where
    [char; N]: Serialize,
    for<'a> [char; N]: Deserialize<'a>,
{
    let corpus = load_corpus(source, data);
    corpus.expand(expansion)
}

fn load_corpus<const N: usize>(source: &str, data: &str) -> NgramData<N>
where
    [char; N]: Serialize,
    for<'a> [char; N]: Deserialize<'a>,
{
    if let Ok(file) = File::open(data) {
        if let Ok(res) = bincode::deserialize_from(file) {
            return res;
        }
        panic!("failed to read ngrams from: {data}");
    }
    println!("failed to open file {data}, it will be created with calculated stats");
    let corpus = std::fs::read_to_string(source).expect("failed to read corpus");
    let corpus = NgramData::<N>::new(corpus.chars());
    let data = File::create(data).expect("failed to create file for ngram data");
    bincode::serialize_into(data, &corpus).expect("save ngram data to file");
    corpus
}

fn expansion(char: char) -> Vec<char> {
    if char.is_ascii_uppercase() {
        vec!['⇧', char.to_ascii_lowercase()]
    } else if char == ' ' {
        vec!['⎵']
    } else {
        vec![char]
    }
}
