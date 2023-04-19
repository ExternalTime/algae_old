pub fn expand_first<T, R, Expansion, Iter, const N: usize>(
    v: &[T; N],
    expand: Expansion,
) -> Vec<[R; N]>
where
    R: Clone,
    Expansion: Fn(&T) -> Iter,
    Iter: IntoIterator<Item = R>,
{
    check_expansion(v, &expand);
    windows(
        expand(&v[0])
            .into_iter()
            .chain(v[1..].iter().flat_map(expand).take(N - 1)),
    )
}

pub fn expand_full<T, R, Expansion, Iter, const N: usize>(
    v: &[T; N],
    expand: Expansion,
) -> Vec<[R; N]>
where
    R: Clone,
    Expansion: Fn(&T) -> Iter,
    Iter: IntoIterator<Item = R>,
{
    check_expansion(v, &expand);
    windows(v.iter().flat_map(expand))
}

fn check_expansion<T, R, Expansion, Iter>(v: &[T], expand: &Expansion)
where
    Expansion: Fn(&T) -> Iter,
    Iter: IntoIterator<Item = R>,
{
    if v.iter().map(expand).any(|i| i.into_iter().next().is_none()) {
        panic!("each expansion must be at least 1 element long");
    }
}

fn windows<R: Clone, const N: usize>(iter: impl Iterator<Item = R>) -> Vec<[R; N]> {
    iter.collect::<Vec<_>>()
        .windows(N)
        .map(|window| match <&[R; N]>::try_from(window) {
            Ok(window) => window.clone(),
            Err(_) => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expansion(_: &()) -> impl Iterator<Item = usize> {
        [0, 1, 2].into_iter()
    }

    #[test]
    fn full_expansion() {
        let a = [(); 2];
        let expanded = expand_full(&a, &expansion);
        assert_eq!(vec![[0, 1], [1, 2], [2, 0], [0, 1], [1, 2]], expanded);
    }

    #[test]
    fn first_expansion() {
        let a = [(); 2];
        let expanded = expand_first(&a, &expansion);
        assert_eq!(vec![[0, 1], [1, 2], [2, 0]], expanded);
    }
}
