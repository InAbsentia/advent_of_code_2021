use std::cmp::{max, max_by, min_by};

#[derive(Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn from_str(input: &str) -> Self {
        let [x, y]: [usize; 2] = input
            .split(",")
            .map(|n| n.parse::<usize>().expect("Not a number!"))
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    fn new((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Self {
        Self {
            start: Coord::new(x1, y1),
            end: Coord::new(x2, y2),
        }
    }

    fn parse(input: &str) -> Self {
        let [start, end]: [Coord; 2] = input
            .split(" -> ")
            .map(Coord::from_str)
            .collect::<Vec<Coord>>()
            .try_into()
            .unwrap();

        Self { start, end }
    }

    fn points(&self) -> Vec<Coord> {
        let start = min_by(&self.start, &self.end, |c1, c2| c1.x.cmp(&c2.x));
        let end = max_by(&self.start, &self.end, |c1, c2| c1.x.cmp(&c2.x));
        let x_range = (start.x..=end.x).into_iter();

        if self.is_diagonal() {
            let increasing = start.y < end.y;

            x_range
                .enumerate()
                .map(|(i, x)| {
                    let y = if increasing { start.y + i } else { start.y - i };
                    Coord::new(x, y)
                })
                .collect()
        } else {
            let y_range = if start.y < end.y {
                start.y..=end.y
            } else {
                end.y..=start.y
            };
            x_range
                .flat_map(|x| y_range.clone().into_iter().map(move |y| Coord::new(x, y)))
                .collect()
        }
    }

    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    points: Vec<Vec<usize>>,
}

impl Map {
    fn new(x: usize, y: usize) -> Self {
        Self {
            points: vec![vec![0; y]; x],
        }
    }

    fn build(lines: &Vec<Line>) -> Self {
        let (max_x, max_y): (usize, usize) = lines.iter().fold((0, 0), |acc, line| {
            (
                max(max(line.start.x, line.end.x) + 1, acc.0),
                max(max(line.start.y, line.end.y) + 1, acc.1),
            )
        });

        Self::new(max_x, max_y)
    }

    fn add_lines(&mut self, lines: &Vec<Line>, include_diagonals: bool) {
        for line in lines {
            if !line.is_diagonal() || include_diagonals {
                for point in line.points() {
                    self.points[point.x][point.y] += 1;
                }
            }
        }
    }

    fn intersection_count(&self) -> usize {
        self.points
            .iter()
            .flat_map(|x| x.iter().map(|y| *y > 1))
            .filter(|i| *i)
            .count()
    }
}

pub fn solve<T: AsRef<str>>(input: &[T]) -> (usize, usize) {
    let lines: Vec<Line> = input.into_iter().map(|s| Line::parse(s.as_ref())).collect();

    (part_one(&lines), part_two(&lines))
}

fn part_one(lines: &Vec<Line>) -> usize {
    let mut map = Map::build(&lines);
    map.add_lines(lines, false);

    map.intersection_count()
}

fn part_two(lines: &Vec<Line>) -> usize {
    let mut map = Map::build(&lines);
    map.add_lines(lines, true);

    map.intersection_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_new() {
        let coord = Coord::new(4, 7);

        assert_eq!(coord, Coord { x: 4, y: 7 });
    }

    #[test]
    fn coord_from_str() {
        let coord = Coord::from_str("0,0");

        assert_eq!(coord, Coord { x: 0, y: 0 });
    }

    #[test]
    fn line_new() {
        let line = Line::new((0, 1), (0, 2));

        assert_eq!(
            line,
            Line {
                start: Coord { x: 0, y: 1 },
                end: Coord { x: 0, y: 2 },
            }
        );
    }

    #[test]
    fn line_parse() {
        let line = Line::parse("0,1 -> 0,2");

        assert_eq!(line, Line::new((0, 1), (0, 2)));
    }

    #[test]
    fn line_points() {
        let horizontal_line = Line::new((0, 1), (0, 3));
        let diagonal_line = Line::new((9, 7), (7, 9));

        assert_eq!(
            horizontal_line.points(),
            vec![Coord::new(0, 1), Coord::new(0, 2), Coord::new(0, 3),]
        );
        assert_eq!(
            diagonal_line.points(),
            vec![Coord::new(7, 9), Coord::new(8, 8), Coord::new(9, 7),]
        );
    }

    #[test]
    fn line_is_diagonal() {
        let vertical_line = Line::new((0, 1), (0, 2));
        let horizontal_line = Line::new((0, 1), (2, 1));
        let diagonal_line = Line::new((0, 1), (1, 2));

        assert!(!vertical_line.is_diagonal());
        assert!(!horizontal_line.is_diagonal());
        assert!(diagonal_line.is_diagonal());
    }

    #[test]
    fn map_new() {
        let map = Map::new(2, 3);

        assert_eq!(map.points, vec![vec![0; 3]; 2]);
    }

    #[test]
    fn map_build() {
        let map = Map::build(&vec![Line::new((1, 1), (1, 8)), Line::new((7, 2), (3, 2))]);

        for point in map.points.iter() {
            assert_eq!(point.len(), 9);
        }
        assert_eq!(map.points.len(), 8);
    }

    #[test]
    fn map_add_lines() {
        let mut map = Map::new(4, 4);

        map.add_lines(
            &vec![
                Line::new((1, 1), (1, 3)),
                Line::new((1, 2), (3, 2)),
                Line::new((2, 1), (3, 3)),
            ],
            false,
        );

        assert_eq!(
            map.points,
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 2, 1],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
            ]
        );
    }

    #[test]
    fn map_intersection_count() {
        let mut map = Map::new(4, 4);

        map.add_lines(
            &vec![
                Line::new((1, 1), (1, 3)),
                Line::new((1, 2), (3, 2)),
                Line::new((0, 3), (3, 3)),
            ],
            false,
        );

        // 0 0 0 0
        // 0 1 2 1
        // 0 0 1 0
        // 1 1 2 1
        assert_eq!(map.intersection_count(), 2);
    }

    #[test]
    fn example_case() {
        let input: Vec<String> = [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let solution = solve(&input);

        assert_eq!(solution, (5, 12));
    }
}
