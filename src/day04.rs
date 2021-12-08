use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
enum Status {
    Marked,
    Clear,
}

#[derive(Clone, Debug, PartialEq)]
struct Square {
    value: i32,
    status: Status,
}

impl Square {
    fn new(value: i32) -> Self {
        Self {
            value,
            status: Status::Clear,
        }
    }

    fn build(value: i32, status: Status) -> Self {
        Self { value, status }
    }
}

type Row = Vec<Square>;

#[derive(Clone, Debug, PartialEq)]
struct Card {
    rows: Vec<Row>,
    winning_number: Option<i32>,
}

impl Card {
    fn new(rows: Vec<Row>) -> Self {
        Self {
            rows,
            winning_number: None,
        }
    }

    fn build(lines: Vec<&str>) -> Self {
        let separator = Regex::new(r"\s+").expect("Invalid regex");

        let rows = lines
            .into_iter()
            .map(|line| {
                separator
                    .split(line)
                    .filter(|&s| s != "")
                    .map(|number| Square::new(number.parse::<i32>().unwrap()))
                    .collect()
            })
            .collect();

        Self {
            rows,
            winning_number: None,
        }
    }

    fn mark(&mut self, number: i32) {
        for row in &mut self.rows {
            for mut square in row {
                if square.value == number {
                    square.status = Status::Marked;
                }
            }
        }
        if self.is_winner() {
            self.winning_number = Some(number);
        }
    }

    fn cols(&self) -> Vec<Vec<Square>> {
        (0..self.rows[0].len())
            .map(|i| self.rows.iter().map(|row| row[i].clone()).collect())
            .collect()
    }

    fn is_winner(&self) -> bool {
        let cols = self.cols();
        let winning_row: Option<&Vec<Square>> = self
            .rows
            .iter()
            .find(|row| row.iter().all(|square| square.status == Status::Marked));
        let winning_col: Option<&Vec<Square>> = cols
            .iter()
            .find(|col| col.iter().all(|square| square.status == Status::Marked));

        winning_row.is_some() || winning_col.is_some()
    }

    fn unmarked(&self) -> Vec<i32> {
        self.rows
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|square| square.status == Status::Clear)
                    .map(|square| square.value)
            })
            .collect()
    }
}

#[derive(Debug)]
struct Winner {
    score: i32,
}

impl Winner {
    fn new() -> Self {
        Self { score: 0 }
    }

    fn calculate_score(card: &Card) -> Self {
        let unmarked_total: i32 = card.unmarked().iter().sum();

        Self {
            score: unmarked_total * card.winning_number.unwrap(),
        }
    }
}

struct Game {
    cards: Vec<Card>,
    numbers: Vec<i32>,
    first_winner: Winner,
    last_winner: Winner,
}

impl Game {
    fn build<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut iter = lines.into_iter();
        let numbers: Vec<i32> = iter
            .nth(0)
            .unwrap()
            .as_ref()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let lines_length = iter.clone().len();

        let (card_inputs, _) = iter
            .map(|s| s.as_ref())
            .enumerate()
            .skip_while(|(_, s)| *s == "")
            .fold((vec![], vec![]), |(mut cards, mut current), (i, line)| {
                if line == "" {
                    cards.push(current);

                    (cards, vec![])
                } else if i + 1 == lines_length {
                    current.push(line);
                    cards.push(current);

                    (cards, vec![])
                } else {
                    current.push(line);

                    (cards, current)
                }
            });

        let cards: Vec<_> = card_inputs.into_iter().map(Card::build).collect();

        Self {
            cards,
            numbers,
            first_winner: Winner::new(),
            last_winner: Winner::new(),
        }
    }

    fn run(&mut self) {
        let mut winners: Vec<Card> = vec![];
        let mut cards = self.cards.clone();
        let num_cards = cards.len();

        'outer: for n in &self.numbers {
            let mut next_cards: Vec<Card> = vec![];

            for mut card in cards {
                card.mark(*n);

                if let Some(_) = card.winning_number {
                    winners.push(card.clone());

                    if winners.len() == num_cards {
                        break 'outer;
                    }
                } else {
                    next_cards.push(card);
                }
            }

            cards = next_cards.clone();
        }

        let first_winner = winners.first().expect("No winners!");
        let last_winner = winners.last().expect("No winners!");

        self.first_winner = Winner::calculate_score(first_winner);
        self.last_winner = Winner::calculate_score(last_winner);
    }
}

pub fn solve<T: AsRef<str>>(lines: &[T]) -> (i32, i32) {
    let mut game = Game::build(lines);
    game.run();

    (game.first_winner.score, game.last_winner.score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_building() {
        let expected = Card::new(vec![
            vec![Square::new(14), Square::new(21)],
            vec![Square::new(10), Square::new(9)],
        ]);

        let card = Card::build(vec!["14 21", "10  9"]);

        assert_eq!(card, expected);
    }

    #[test]
    fn card_cols() {
        assert_eq!(
            build_test_card().cols(),
            vec![
                vec![Square::new(14), Square::new(10)],
                vec![Square::new(21), Square::new(9)],
            ]
        )
    }

    #[test]
    fn card_marking() {
        let mut card = build_test_card();

        card.mark(9);

        assert_eq!(card.rows[1][1], Square::build(9, Status::Marked));
    }

    #[test]
    fn card_is_winner_unmarked() {
        let card = build_test_card();

        assert!(!card.is_winner());
    }

    #[test]
    fn card_is_winner_winning_row() {
        let mut card = build_test_card();

        assert!(!card.is_winner());

        card.mark(14);
        card.mark(21);

        assert!(card.is_winner());
    }

    #[test]
    fn card_is_winner_winning_col() {
        let mut card = build_test_card();

        card.mark(9);
        card.mark(21);

        assert!(card.is_winner());
    }

    #[test]
    fn card_unmarked() {
        let card = build_test_card();

        assert_eq!(card.unmarked(), vec![14, 21, 10, 9]);
    }

    #[test]
    fn example_case() {
        let input: Vec<String> = [
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let solution = solve(&input);

        assert_eq!(solution, (4512, 1924));
    }

    fn build_test_card() -> Card {
        Card::new(vec![
            vec![Square::new(14), Square::new(21)],
            vec![Square::new(10), Square::new(9)],
        ])
    }
}
