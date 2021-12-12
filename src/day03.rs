pub fn solve<T: AsRef<str>>(lines: &[T]) -> (usize, usize) {
    (part_one(&lines) as usize, part_two(&lines) as usize)
}

fn part_one<T: AsRef<str>>(lines: &[T]) -> i32 {
    let aggregates = parse_aggregates(lines);
    let mcd: Vec<u8> = most_common_digits(&aggregates);
    let lcd: Vec<u8> = mcd.iter().map(|i| *i ^ 1).collect();

    let gamma = binary_vec_to_i32(&mcd);
    let epsilon = binary_vec_to_i32(&lcd);

    gamma * epsilon
}

enum LifeSupportRating {
    OxygenGeneratorRating,
    CO2ScrubberRating,
}

fn part_two<T: AsRef<str>>(lines: &&[T]) -> i32 {
    get_life_support_rating(lines, LifeSupportRating::OxygenGeneratorRating)
        * get_life_support_rating(lines, LifeSupportRating::CO2ScrubberRating)
}

fn get_life_support_rating<T: AsRef<str>>(lines: &&[T], mapper_fn: LifeSupportRating) -> i32 {
    let mut i = 0;
    let mut keep: Vec<&str> = lines.iter().map(|l| l.as_ref()).collect();

    let result = loop {
        let aggregates = parse_aggregates(&keep);
        let mapper: Vec<u8> = match mapper_fn {
            LifeSupportRating::OxygenGeneratorRating => most_common_digits(&aggregates),
            LifeSupportRating::CO2ScrubberRating => least_common_digits(&aggregates),
        };
        let m = mapper[i];

        keep = keep
            .into_iter()
            .map(|l| l.as_ref())
            .filter(|l: &&str| {
                l.chars().nth(i).unwrap() == std::char::from_digit(m.into(), 10).unwrap()
            })
            .collect();

        if keep.len() == 1 {
            break keep.into_iter().nth(0).unwrap();
        } else {
            i += 1;
        }
    };

    i32::from_str_radix(result, 2).unwrap()
}

fn parse_aggregates<T: AsRef<str>>(lines: &[T]) -> Vec<Vec<u32>> {
    let iter = lines.into_iter();
    let length = lines[0].as_ref().len();

    let mut aggregates: Vec<Vec<u32>> = vec![Vec::new(); length];
    iter.for_each(|line| {
        line.as_ref().chars().enumerate().for_each(|(i, digit)| {
            let value = digit.to_string().parse::<u32>().unwrap();

            aggregates.get_mut(i).unwrap().push(value);
        });
    });

    aggregates
}

fn most_common_digits(aggregates: &Vec<Vec<u32>>) -> Vec<u8> {
    aggregates
        .clone()
        .into_iter()
        .map(|digits| {
            let sum: u32 = digits.clone().into_iter().sum();

            if (sum as f64) < (digits.len() as f64 / 2.0).ceil() {
                0
            } else {
                1
            }
        })
        .collect()
}

fn least_common_digits(aggregates: &Vec<Vec<u32>>) -> Vec<u8> {
    aggregates
        .clone()
        .into_iter()
        .map(|digits| {
            let sum: u32 = digits.clone().into_iter().sum();

            if (sum as f64) < (digits.len() as f64 / 2.0).ceil() {
                1
            } else {
                0
            }
        })
        .collect()
}

fn binary_vec_to_i32(binary: &Vec<u8>) -> i32 {
    binary.iter().fold(0, |acc, &b| acc * 2 + b as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input: Vec<String> = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let solution = solve(&input);

        assert_eq!(solution, (198, 230));
    }
}
