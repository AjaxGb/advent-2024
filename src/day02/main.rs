use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_record(record: &[i32]) -> bool {
    let valid_diffs = if record[0] < record[1] {
        1..=3
    } else {
        -3..=-1
    };
    record
        .iter()
        .copied()
        .tuple_windows()
        .all(|(a, b)| valid_diffs.contains(&(b - a)))
}

fn find_p1<'a>(records: impl Iterator<Item = &'a [i32]>) -> usize {
    records.filter(|&s| is_safe_record(s)).count()
}

fn main() {
    let records = parse_input(include_str!("input.txt"));
    let p1 = find_p1(records.iter().map(Vec::as_slice));
    println!("P1: {p1}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_example() {
        let records = parse_input(include_str!("example.txt"));
        assert_eq!(find_p1(records.iter().map(Vec::as_slice)), 2);
    }
}
