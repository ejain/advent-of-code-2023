use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;
use counter::Counter;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/02.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> u32 {
    let bag = HashMap::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14)
    ]);
    let mut sum = 0;
    for game in parse_games(input) {
        if is_game_possible(&game, &bag) {
            sum += game.id
        }
    }
    sum
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines()
        .map(&str::trim)
        .filter(|line| !line.is_empty())
        .map(parse_game)
        .collect()
}

fn parse_game(line: &str) -> Game {
    let mut split = line.split(": ");
    let prefix = split.next().unwrap();
    let rest = split.next().unwrap();
    let re = regex!(r"Game (\d+)");
    let c = re.captures(prefix).unwrap();
    let id = c[1].parse::<u32>().unwrap();
    Game::new(id, parse_draws(rest))
}

fn parse_draws(s: &str) -> Vec<Counter<Color>> {
    s.split("; ").map(parse_draw).collect()
}

fn parse_draw(s: &str) -> Counter<Color> {
    let mut draw: Counter<Color> = Counter::new();
    for token in s.split(", ") {
        let re = regex!(r"(\d+) (\w+)");
        let c = re.captures(token).unwrap();
        let count = c[1].parse::<usize>().unwrap();
        let color = match &c[2] {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => panic!("unsupported color")
        };
        draw.insert(color, count);
    }
    draw
}

fn is_game_possible(game: &Game, bag: &HashMap<Color, usize>) -> bool {
    game.draws.iter().all(|draw| is_draw_possible(draw, bag))
}

fn is_draw_possible(draw: &Counter<Color>, bag: &HashMap<Color, usize>) -> bool {
    [Color::Red, Color::Green, Color::Blue].iter().all(|color| {
        let used_cubes = draw.get(color).unwrap_or(&0);
        let total_cubes = bag.get(color).unwrap_or(&0);
        used_cubes <= total_cubes
    })
}

fn solve_part_2(input: &str) -> usize {
    parse_games(input).iter().map(get_power).sum()
}

fn get_power(game: &Game) -> usize {
    let mut max_red_cubes = 1;
    let mut max_green_cubes = 1;
    let mut max_blue_cubes = 1;
    for draw in &game.draws {
        if let Some(red_cubes) = draw.get(&Color::Red) {
            max_red_cubes = max(max_red_cubes, *red_cubes)
        }
        if let Some(green_cubes) = draw.get(&Color::Green) {
            max_green_cubes = max(max_green_cubes, *green_cubes)
        }
        if let Some(blue_cubes) = draw.get(&Color::Blue) {
            max_blue_cubes = max(max_blue_cubes, *blue_cubes)
        }
    }
    max_red_cubes * max_green_cubes * max_blue_cubes
}

#[derive(PartialEq, Eq, Hash)]
enum Color { Red, Blue, Green }

struct Game {
    id: u32,
    draws: Vec<Counter<Color>>,
}

impl Game {
    fn new(id: u32, draws: Vec<Counter<Color>>) -> Game {
        Game { id, draws }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_game() {
        let actual = parse_game("Game 42: 3 blue, 1 red; 1 red, 2 green, 1 blue");
        assert_eq!(actual.id, 42, "game id");
        assert_eq!(actual.draws.len(), 2, "number of draws");
        assert_eq!(actual.draws[0].get(&Color::Red), Some(&1), "red cubes in first draw");
        assert_eq!(actual.draws[0].get(&Color::Blue), Some(&3), "blue cubes in first draw");
        assert_eq!(actual.draws[0].get(&Color::Green), None, "green cubes in first draw");
        assert_eq!(actual.draws[1].get(&Color::Red), Some(&1), "red cubes in second draw");
        assert_eq!(actual.draws[1].get(&Color::Blue), Some(&1), "blue cubes in second draw");
        assert_eq!(actual.draws[1].get(&Color::Green), Some(&2), "green cubes in second draw");
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1("
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "), 8);
    }

    #[test]
    fn test_get_power() {
        let actual = parse_game("Game 42: 3 blue, 2 red; 1 red, 4 green, 1 blue");
        assert_eq!(get_power(&actual), 3 * 2 * 4);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2("
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "), 2286);
    }
}
