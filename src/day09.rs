#[derive(Debug, PartialEq)]
struct HeightMap(Vec<Vec<i8>>);

impl HeightMap {
    fn parse<T: AsRef<str>>(input: &[T]) -> Self {
        let map = input
            .iter()
            .map(|l| {
                l.as_ref()
                    .split("")
                    .filter(|s| s != &"")
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();

        Self(map)
    }

    fn low_points(&self) -> Vec<i8> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter().enumerate().filter(move |(y, point)| {
                    if **point == 9 {
                        return false;
                    }

                    let mut neighbors: Vec<(usize, usize)> = Vec::new();

                    if x > 0 {
                        neighbors.push((x - 1, *y));
                    }
                    neighbors.push((x + 1, *y));
                    if *y > 0 {
                        neighbors.push((x, y - 1));
                    }
                    neighbors.push((x, y + 1));

                    neighbors.iter().all(|(nx, ny)| {
                        if let Some(r) = self.0.get(*nx) {
                            if let Some(n) = r.get(*ny) {
                                n > point
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    })
                })
            })
            .map(|(_, val)| *val)
            .collect()
    }
}

pub fn solve<T: AsRef<str>>(input: &[T]) -> (usize, usize) {
    let height_map = HeightMap::parse(input);

    (part_one(&height_map), part_two(&height_map))
}

fn part_one(input: &HeightMap) -> usize {
    let low_points = input.low_points();

    low_points.into_iter().map(|n| n as usize + 1).sum()
}

fn part_two(_input: &HeightMap) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn height_map_parse() {
        let input = ["123", "456", "789"];
        let expected = HeightMap(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        let actual = HeightMap::parse(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_case() {
        let input: Vec<String> = [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let solution = solve(&input);

        assert_eq!(solution, (15, 0));
    }
}
