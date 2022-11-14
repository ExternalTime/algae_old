pub fn hill_climb<Score: Ord>(
    layout: &mut [usize],
    pinned: usize,
    analyze: impl Fn(&[usize]) -> Score,
) -> Score {
    let mut best_score = analyze(layout);
    loop {
        let mut last = true;
        for i in pinned..layout.len() {
            for j in (i + 1)..layout.len() {
                layout.swap(i, j);
                let score = analyze(layout);
                if score < best_score {
                    best_score = score;
                    last = false;
                } else {
                    layout.swap(i, j);
                }
            }
        }
        if last {
            break;
        }
    }
    best_score
}
