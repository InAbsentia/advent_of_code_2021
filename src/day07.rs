use std::cmp::{max, min};

pub fn solve<T: AsRef<str>>(input: &[T]) -> (usize, usize) {
    let positions = parse_input(input);

    (part_one(&positions), part_two(&positions))
}

fn part_one(positions: &Vec<usize>) -> usize {
    let highest_position = positions.iter().max().unwrap();

    (0..=*highest_position)
        .map(|p| positions.into_iter().map(|c| max(c, &p) - min(c, &p)).sum())
        .min()
        .unwrap()
}

fn part_two(positions: &Vec<usize>) -> usize {
    let highest_position = positions.iter().max().unwrap();

    (0..=*highest_position)
        .map(|p| {
            positions
                .into_iter()
                .map(|c| {
                    let n = max(c, &p) - min(c, &p);
                    ((n * (n + 1)) / 2) as usize
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn parse_input<T: AsRef<str>>(input: &[T]) -> Vec<usize> {
    input
        .first()
        .unwrap()
        .as_ref()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input: Vec<String> = ["16,1,2,0,4,2,7,1,2,14"]
            .into_iter()
            .map(String::from)
            .collect();

        let solution = solve(&input);

        assert_eq!(solution, (37, 168));
    }
}
