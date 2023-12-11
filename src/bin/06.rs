use std::cmp::max;

fn main() {
    println!("Part 1: {}", solve_part_1(&[
        Race::new(44, 277),
        Race::new(89, 1136),
        Race::new(96, 1890),
        Race::new(91, 1768),
    ]));
    println!("Part 2: {}", solve_part_2(
        &Race::new(44899691, 277113618901768)
    ));
}

fn solve_part_1(races: &[Race]) -> u64 {
    let mut product: u64 = 1;
    for race in races {
        product *= max(1, race.find_faster_speeds())
    }
    product
}

fn solve_part_2(race: &Race) -> u64 {
    race.find_faster_speeds()
}

struct Race {
    duration: u64,
    best_distance: u64,
}

impl Race {
    fn new(duration: u64, best_distance: u64) -> Race {
        Race { duration, best_distance }
    }

    fn find_faster_speeds(&self) -> u64 {
        let mut faster_speeds = 0;
        for speed in 1..self.duration - 1 {
            let distance = (self.duration - speed) * speed;
            if distance > self.best_distance {
                faster_speeds += 1;
            }
        }
        faster_speeds
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_faster_speeds() {
        let race = Race::new(7, 9);
        assert_eq!(race.find_faster_speeds(), 4);
    }

    #[test]
    fn test_solve_part_1() {
        let races = vec![
            Race::new(7, 9),
            Race::new(15, 40),
            Race::new(30, 200),
        ];
        assert_eq!(solve_part_1(&races), 288);
    }

    #[test]
    fn test_solve_part_2() {
        let race = Race::new(71530, 940200);
        assert_eq!(solve_part_2(&race), 71503);
    }
}
