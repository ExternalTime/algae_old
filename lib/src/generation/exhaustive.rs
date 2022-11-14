pub fn exhaustive(layout: &mut [usize], pinned: usize, step_score: impl Fn(&[usize]) -> u64) {
    let len = layout.len();
    let mut stack = Vec::new();
    stack.push((pinned, pinned, 0));
    let mut best_score = u64::MAX;
    let mut best_layout: Vec<_> = Vec::new();
    while let Some((i1, i2, score)) = stack.pop() {
        if len <= i2 {
            // Cleanup
            layout[i1..].rotate_left(1);
            continue;
        }
        layout.swap(i1, i2);

        // Next sibling
        stack.push((i1, i2 + 1, score));

        let score = score + step_score(&layout[..i1 + 1]);
        if score < best_score {
            if i1 < len - 1 {
                // First child
                stack.push((i1 + 1, i1 + 1, score));
            } else {
                best_score = score;
                best_layout = layout.into();
                continue;
            }
        }
    }
    assert_eq!(
        len,
        best_layout.len(),
        "exhaustive search somehow failed to find any layouts at all"
    );
    for (v1, v2) in layout.iter_mut().zip(best_layout.into_iter()) {
        *v1 = v2;
    }
}
