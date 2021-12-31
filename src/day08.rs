use std::collections::{BTreeSet, HashMap, HashSet};

type WireSet = BTreeSet<char>;

#[derive(Debug, PartialEq)]
struct Panel {
    numbers: HashSet<WireSet>,
    displays: [WireSet; 4],
}

fn wire_set_from_str(input: &str) -> WireSet {
    let mut set = BTreeSet::new();

    for c in input.chars() {
        set.insert(c);
    }

    set
}

impl Panel {
    fn from_str(input: &str) -> Self {
        let (num_str, disp_str) = input.split_once(" | ").unwrap();
        let numbers = HashSet::from_iter(num_str.split_whitespace().map(wire_set_from_str));
        let displays = disp_str
            .split_whitespace()
            .map(wire_set_from_str)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong number of displays!");

        Self { numbers, displays }
    }
}

pub fn solve<T: AsRef<str>>(input: &[T]) -> (usize, usize) {
    let panels: Vec<Panel> = parse_input(input);

    (part_one(&panels), part_two(panels))
}

fn part_one(panels: &Vec<Panel>) -> usize {
    panels
        .iter()
        .map(|panel| {
            panel
                .displays
                .iter()
                .filter(|p| [2, 3, 4, 7].contains(&p.len()))
                .count()
        })
        .sum()
}

fn part_two(panels: Vec<Panel>) -> usize {
    panels
        .into_iter()
        .map(
            |Panel {
                 mut numbers,
                 displays,
             }|
             -> usize {
                let mut wire_numbers: HashMap<u8, &WireSet> = HashMap::new();

                let one = numbers.iter().find(|s| s.len() == 2).unwrap().clone();
                wire_numbers.insert(1, &one);
                numbers.remove(&one);

                let four = numbers.iter().find(|s| s.len() == 4).unwrap().clone();
                wire_numbers.insert(4, &four);
                numbers.remove(&four);

                let seven = numbers.iter().find(|s| s.len() == 3).unwrap().clone();
                wire_numbers.insert(7, &seven);
                numbers.remove(&seven);

                let eight = numbers.iter().find(|s| s.len() == 7).unwrap().clone();
                wire_numbers.insert(8, &eight);
                numbers.remove(&eight);

                let nine = numbers
                    .iter()
                    .find(|s| s.len() == 6 && s.is_superset(wire_numbers[&4]))
                    .unwrap()
                    .clone();
                wire_numbers.insert(9, &nine);
                numbers.remove(&nine);

                let zero = numbers
                    .iter()
                    .find(|s| s.len() == 6 && s.is_superset(wire_numbers[&1]))
                    .unwrap()
                    .clone();
                wire_numbers.insert(0, &zero);
                numbers.remove(&zero);

                let six = numbers.iter().find(|s| s.len() == 6).unwrap().clone();
                wire_numbers.insert(6, &six);
                numbers.remove(&six);

                let three = numbers
                    .iter()
                    .find(|s| s.len() == 5 && s.is_superset(wire_numbers[&1]))
                    .unwrap()
                    .clone();
                wire_numbers.insert(3, &three);
                numbers.remove(&three);

                let five = numbers
                    .iter()
                    .find(|s| s.len() == 5 && s.is_subset(wire_numbers[&6]))
                    .unwrap()
                    .clone();
                wire_numbers.insert(5, &five);
                numbers.remove(&five);

                let two = numbers.iter().find(|s| s.len() == 5).unwrap().clone();
                wire_numbers.insert(2, &two);
                numbers.remove(&two);

                displays
                    .iter()
                    .map(|wires| {
                        wire_numbers
                            .iter()
                            .find(|(_, s)| **s == wires)
                            .unwrap()
                            .0
                            .to_string()
                    })
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            },
        )
        .sum()
}

fn parse_input<T: AsRef<str>>(input: &[T]) -> Vec<Panel> {
    input
        .into_iter()
        .map(|s| s.as_ref())
        .map(Panel::from_str)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_from_str() {
        let input = BTreeSet::from_iter("abcefg".chars());

        assert_eq!(wire_set_from_str("fgcbea"), input);
    }

    #[test]
    fn panel_from_str() {
        let numbers = HashSet::from(
            [
                "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd",
                "edb",
            ]
            .map(wire_set_from_str),
        );
        let displays = ["fdgacbe", "cefdb", "cefbgd", "gcbe"].map(wire_set_from_str);

        assert_eq!(
            Panel::from_str("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            Panel { numbers, displays }
        );
    }

    #[test]
    fn example_case() {
        let input: Vec<String> = [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ]
            .into_iter()
            .map(String::from)
            .collect();

        let solution = solve(&input);

        assert_eq!(solution, (26, 61229));
    }
}
