use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

type Balls = HashMap<Color, u32>;

struct Game {
    id: u32,
    subgames: Vec<Balls>,
}

fn part_one(input: &[Game], loaded_balls: Balls) -> u32 {
    input
        .iter()
        .filter(|game| {
            game.subgames.iter().all(|balls| {
                balls
                    .iter()
                    .all(|(color, count)| loaded_balls.get(color).unwrap() >= count)
            })
        })
        .map(|game| game.id)
        .sum()
}

fn part_two(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            let mut min_balls: Balls = HashMap::new();
            game.subgames.iter().for_each(|subgame| {
                subgame.iter().for_each(|(color, n)| {
                    min_balls
                        .entry(color.clone())
                        .and_modify(|current_n| *current_n = std::cmp::max(*current_n, *n))
                        .or_insert(*n);
                });
            });
            min_balls.iter().fold(1, |acc, (_, n)| acc * n)
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut separated_line = line.split(':');
            let id: u32 = separated_line
                .next()
                .unwrap()
                .split(' ')
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let subgames: Vec<Balls> = separated_line
                .next()
                .unwrap()
                .split(';')
                .map(|subgame| {
                    let x: Balls = subgame
                        .split(',')
                        .map(|ball| {
                            let mut ball = ball.split_whitespace();
                            let count: u32 = ball.next().unwrap().parse().unwrap();
                            let color = match ball.next().unwrap() {
                                "red" => Color::Red,
                                "green" => Color::Green,
                                "blue" => Color::Blue,
                                _ => panic!("Invalid color"),
                            };
                            (color, count)
                        })
                        .collect();
                    x
                })
                .collect();
            Game { id, subgames }
        })
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let parsed_input = parse_input(input);
    let loaded_balls = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    let output_one = part_one(&parsed_input, loaded_balls);
    let output_two = part_two(&parsed_input);
    println!("Part one: {}", output_one);
    println!("Part two: {}", output_two);
}

#[cfg(test)]
mod test {
    use super::*;
    const GAME: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part_one() {
        let loaded_balls: HashMap<Color, u32> =
            HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
        const EXPECTED_OUTPUT: u32 = 8;

        let parsed_input = parse_input(GAME);
        let output = part_one(&parsed_input, loaded_balls);
        assert_eq!(output, EXPECTED_OUTPUT);
    }

    #[test]
    fn test_part_two() {
        const EXPECTED_OUTPUT: u32 = 2286;

        let parsed_input = parse_input(GAME);
        let output = part_two(&parsed_input);
        assert_eq!(output, EXPECTED_OUTPUT);
    }
}
