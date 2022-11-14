pub fn not_quite_anneal(
    layout: &mut [usize],
    pinned: usize,
    analyze: impl Fn(&[usize]) -> u64,
    decay_resistance: u64,
) -> u64 {
    let mut current_score = analyze(layout);
    debug_assert!(pinned + 1 < layout.len());
    let mut temperature = current_score;
    while 0 < temperature {
        let i1 = fastrand::usize(pinned..layout.len());
        let mut i2 = i1;
        while i2 == i1 {
            i2 = fastrand::usize(pinned..layout.len());
        }
        layout.swap(i1, i2);
        let new_score = analyze(layout);
        if new_score < current_score + temperature {
            temperature = temperature + current_score - new_score;
            current_score = new_score;
        } else {
            layout.swap(i1, i2);
        }
        temperature = (temperature * decay_resistance) / (decay_resistance + 1);
    }
    current_score
}
