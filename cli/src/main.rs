mod corpus;
mod layout;

use algae_lib::*;
use corpus::get_corpus;
use layout::{Key, Layout};

fn main() {
    let mut args = std::env::args().skip(1);
    if let [Some(corpus), Some(saved)] = [args.next(), args.next()] {
        generate(corpus.as_ref(), saved.as_ref());
    } else {
        println!("please input a path to corpus and where to save its stats");
    }
}

fn generate(corpus: &str, saved: &str) {
    let bigrams = get_corpus::<2>(corpus, saved).ngrams::<2>();
    println!("loaded the corpus");
    let generator = Generator::new(
        (0..3).flat_map(|row| (0..10).map(move |col| Key([row, col]))),
        ",./;".chars().chain('a'..='z'),
        [bigrams],
        sfb_distance,
    )
    .unwrap();
    println!("Generating layout optimized for sfb distance (taxicab).");
    let pins = [];
    let result: Layout = generator.generate(pins).unwrap();
    println!("{result}");
}

fn sfb_distance([Key([y1, x1]), Key([y2, x2])]: [Key; 2]) -> u64 {
    if x1 == x2 {
        return y1.abs_diff(y2) as u64;
    }
    let indexes = [3..5, 5..7];
    for index in indexes {
        if index.contains(&x1) && index.contains(&x2) {
            return 1 + y1.abs_diff(y2) as u64;
        }
    }
    0
}
