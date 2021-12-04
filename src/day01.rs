pub fn solve(input: Vec<String>) -> (i32, i32) {
    let mut iter = input.into_iter().map(|s| s.parse::<i32>().unwrap());
    let first = iter.nth(0).unwrap();

    let (part_one, _) = iter.fold(
        (0, first),
        |(acc, prev), i| if i > prev { (acc + 1, i) } else { (acc, i) },
    );

    (part_one, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = [
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let solution = solve(input);

        assert_eq!(solution, (7, 0));
    }
}
