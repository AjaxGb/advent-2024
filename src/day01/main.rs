use std::collections::HashMap;

fn expect_two_u32<'a>(mut it: impl Iterator<Item = &'a str>) -> (u32, u32) {
    let a = it.next().unwrap().parse().unwrap();
    let b = it.next().unwrap().parse().unwrap();
    assert!(it.next().is_none(), "more than two items");
    (a, b)
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|s| expect_two_u32(s.split_ascii_whitespace()))
        .unzip()
}

fn find_p1(mut ids_a: Vec<u32>, mut ids_b: Vec<u32>) -> u32 {
    ids_a.sort();
    ids_b.sort();
    core::iter::zip(ids_a, ids_b)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn find_p2(ids_a: &[u32], ids_b: &[u32]) -> u32 {
    let mut counts = HashMap::<u32, u32>::new();
    for &id in ids_b {
        *counts.entry(id).or_default() += 1;
    }
    let counts = counts;
    ids_a
        .iter()
        .map(|id| *id * counts.get(id).copied().unwrap_or_default())
        .sum()
}

fn main() {
    let (ids_a, ids_b) = parse_input(include_str!("input.txt"));
    let p2 = find_p2(&ids_a, &ids_b);
    let p1 = find_p1(ids_a, ids_b);
    println!("P1: {p1}");
    println!("P2: {p2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_example() {
        let (ids_a, ids_b) = parse_input(include_str!("example.txt"));
        assert_eq!(find_p2(&ids_a, &ids_b), 31);
        assert_eq!(find_p1(ids_a, ids_b), 11);
    }
}
