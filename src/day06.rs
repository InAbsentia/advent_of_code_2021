const STARTING_AGE: usize = 8;
const RESET_AGE: usize = 6;

pub fn solve<T: AsRef<str>>(input: &[T]) -> (usize, usize) {
    let ages = parse_ages(input);

    (part_one(&ages), part_two(&ages))
}

fn part_one(ages: &Vec<u8>) -> usize {
    fish_after_days(ages, 80)
}

fn part_two(ages: &Vec<u8>) -> usize {
    fish_after_days(ages, 256)
}

fn fish_after_days(fish: &Vec<u8>, days: u16) -> usize {
    let mut ages: Vec<usize> = vec![0; 9];
    for f in fish {
        ages[*f as usize] += 1;
    }

    for _ in 1..=days {
        let newborns = ages[0];

        for i in 0..STARTING_AGE {
            ages[i] = ages[i + 1];
        }

        ages[RESET_AGE] += newborns;
        ages[STARTING_AGE] = newborns;
    }

    ages.iter().sum()
}

fn parse_ages<T: AsRef<str>>(input: &[T]) -> Vec<u8> {
    input
        .first()
        .unwrap()
        .as_ref()
        .split(",")
        .map(|s| s.parse::<u8>().expect("Not a number!"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input: Vec<String> = ["3,4,3,1,2"].into_iter().map(String::from).collect();

        let solution = solve(&input);

        assert_eq!(solution, (5934, 26984457539));
    }
}
