pub fn solve<T: AsRef<str>>(lines: &[T]) -> (i32, i32) {
    let commands: Vec<(String, i32)> = lines
        .into_iter()
        .map(|l| -> (String, i32) {
            let (command, quantity) = l.as_ref().split_once(" ").unwrap();
            (String::from(command), quantity.parse::<i32>().unwrap())
        })
        .collect();

    (part_one(&commands), part_two(&commands))
}

fn part_one(lines: &Vec<(String, i32)>) -> i32 {
    let (position, depth): (i32, i32) = lines.into_iter().fold(
        (0, 0),
        |(position, depth), (command, distance)| match command.as_str() {
            "forward" => (position + distance, depth),
            "down" => (position, depth + distance),
            "up" => (position, depth - distance),
            _ => {
                println!("Unknown command: {:?}!", command);
                (position, depth)
            }
        },
    );

    position * depth
}

fn part_two(lines: &Vec<(String, i32)>) -> i32 {
    let (position, depth, _): (i32, i32, _) =
        lines
            .into_iter()
            .fold(
                (0, 0, 0),
                |(position, depth, aim), (command, distance)| match command.as_str() {
                    "forward" => (position + distance, depth + (aim * distance), aim),
                    "down" => (position, depth, aim + distance),
                    "up" => (position, depth, aim - distance),
                    _ => {
                        println!("Unknown command: {:?}!", command);
                        (position, depth, aim)
                    }
                },
            );

    position * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input: Vec<String> = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let solution = solve(&input);

        assert_eq!(solution, (150, 900));
    }
}
