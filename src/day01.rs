pub fn solve<T: AsRef<str>>(input: &[T]) -> (usize, usize) {
    let ints = to_ints(input);
    let part_one = sum_increases(ints);

    let windowed: Vec<i32> = input
        .windows(3)
        .map(|window| to_ints(window).into_iter().sum())
        .collect();
    let part_two = sum_increases(windowed);

    (part_one as usize, part_two as usize)
}

fn to_ints<T: AsRef<str>>(values: &[T]) -> Vec<i32> {
    values
        .into_iter()
        .map(|s| s.as_ref().parse::<i32>().unwrap())
        .collect()
}

fn sum_increases(values: Vec<i32>) -> i32 {
    let mut iter = values.into_iter();
    let first = iter.by_ref().nth(0).unwrap();
    let (result, _) = iter.fold(
        (0, first),
        |(acc, prev), i| if i > prev { (acc + 1, i) } else { (acc, i) },
    );

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input: Vec<String> = [
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let solution = solve(&input);

        assert_eq!(solution, (7, 5));
    }
}
