mod corpus;

use algae_lib::*;
use corpus::get_corpus;
use std::collections::BTreeMap;

fn main() {
    let mut args = std::env::args().skip(1);
    if let [Some(corpus), Some(saved)] = [args.next(), args.next()] {
        generate(corpus.as_ref(), saved.as_ref());
    } else {
        println!("please input a path to corpus and where to save its stats");
    }
}

fn generate(corpus: &str, saved: &str) {
    let ngrams = get_corpus::<2>(corpus, saved);
    println!("loaded the corpus");
    let ngrams = ngrams.into_inner();
    let total = ngrams.iter().map(|(_, count)| *count).sum::<u64>() as f64;
    let encoding = MapEncoding::new(
        "etaoinshrldcumfpgwybvkxjqz,./;".chars(),
        0usize..30,
        [
            (0, 'd'),
            (1, 'v'),
            (2, 'o'),
            (3, 'r'),
            (4, 'a'),
            (5, 'k'),
            (6, 'b'),
            (7, 'e'),
            (8, 's'),
            (9, 't'),
        ]
        .into_iter()
        .map(|(x, y)| (y, x)),
    );
    let analyzer =
        encoding.weight_calculator(|ngram| ngrams.get(&ngram).copied().unwrap_or(0), weight);

    let mut layout: Vec<usize> = (0..30).collect();

    fastrand::shuffle(&mut layout[10..]);
    generation::hill_climb(&mut layout, 10, |l| analyzer.score(l));
    generation::exhaustive(&mut layout, 10, |l| analyzer.step_score(l));

    println!(
        "sfb distance (taxicab): {}",
        analyzer.score(&layout) as f64 / total
    );
    print_layout(encoding.decode(layout.clone()).map(|(x, y)| (y, x)));
}

fn print_layout(layout: impl Iterator<Item = (usize, char)>) {
    let layout: BTreeMap<_, _> = layout.collect();
    for (index, char) in layout.into_iter() {
        match index % 10 {
            9 => println!("{char}"),
            5 => print!(" {char} "),
            _ => print!("{char} "),
        }
    }
}

fn finger(key: usize) -> Finger {
    let mut key = key % 10;
    let hand = if key < 5 {
        Hand::Left
    } else {
        key = 9 - key;
        Hand::Right
    };
    use FingerKind::*;
    let kind = match key {
        0 => Pinky,
        1 => Ring,
        2 => Middle,
        3 | 4 => Index,
        _ => unreachable!(),
    };
    Finger { hand, kind }
}

fn weight([k1, k2]: [usize; 2]) -> u64 {
    if finger(k1) != finger(k2) {
        return 0;
    }
    ((k1 / 10).abs_diff(k2 / 10) + (k1 % 10).abs_diff(k2 % 10)) as u64
}
