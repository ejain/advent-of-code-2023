use std::cmp::min;
use std::fs::read_to_string;
use rangemap::{RangeMap, RangeSet};

fn main() {
    let input = read_to_string("data/05.txt").unwrap();
    let almanac = Almanac::parse(&input);
    println!("Part 1: {}", solve_part_1(&almanac));
    println!("Part 2: {}", solve_part_2(&almanac));
}

fn solve_part_1(almanac: &Almanac) -> u64 {
    let mut location = u64::MAX;
    for seed in &almanac.seeds {
        location = min(location, almanac.seed_to_location(*seed))
    }
    location
}

fn solve_part_2(almanac: &Almanac) -> u64 {
    let mut location = u64::MAX;
    for range in as_ranges(&almanac.seeds) {
        for seed in range {
            location = min(location, almanac.seed_to_location(seed));
        }
    }
    location
}

fn as_ranges(range_specs: &[u64]) -> RangeSet<u64> {
    let mut ranges = RangeSet::new();
    for i in (0..range_specs.len()).step_by(2) {
        ranges.insert(range_specs[i]..range_specs[i] + range_specs[i + 1]);
    }
    ranges
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: RangeMap<u64, i64>,
    soil_to_fertilizer: RangeMap<u64, i64>,
    fertilizer_to_water: RangeMap<u64, i64>,
    water_to_light: RangeMap<u64, i64>,
    light_to_temperature: RangeMap<u64, i64>,
    temperature_to_humidity: RangeMap<u64, i64>,
    humidity_to_location: RangeMap<u64, i64>,
}

impl Almanac {
    fn parse(input: &str) -> Almanac {
        let mut almanac = Almanac::new();
        let mut current_map = &mut almanac.seed_to_soil;
        for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
            if let Some(line) = line.strip_prefix("seeds: ") {
                almanac.seeds = line.split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
            } else if let Some(line) = line.strip_suffix(" map:") {
                match line {
                    "seed-to-soil" => current_map = &mut almanac.seed_to_soil,
                    "soil-to-fertilizer" => current_map = &mut almanac.soil_to_fertilizer,
                    "fertilizer-to-water" => current_map = &mut almanac.fertilizer_to_water,
                    "water-to-light" => current_map = &mut almanac.water_to_light,
                    "light-to-temperature" => current_map = &mut almanac.light_to_temperature,
                    "temperature-to-humidity" => current_map = &mut almanac.temperature_to_humidity,
                    "humidity-to-location" => current_map = &mut almanac.humidity_to_location,
                    _ => panic!("don't know how to map <{}>", line),
                }
            } else {
                let values: Vec<i64> = line.split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                assert_eq!(values.len(), 3, "expected 3 values but got <{}>", line);
                let target_begin = values[0];
                let source_begin = values[1];
                let length = values[2];
                let adjustment = target_begin - source_begin;
                let source_end = source_begin + length;
                current_map.insert(source_begin as u64..source_end as u64, adjustment);
            }
        }
        almanac
    }

    fn new() -> Almanac {
        Almanac {
            seeds: Vec::new(),
            seed_to_soil: RangeMap::new(),
            soil_to_fertilizer: RangeMap::new(),
            fertilizer_to_water: RangeMap::new(),
            water_to_light: RangeMap::new(),
            light_to_temperature: RangeMap::new(),
            temperature_to_humidity: RangeMap::new(),
            humidity_to_location: RangeMap::new(),
        }
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        let mut seed = seed;
        seed = self.adjust(seed, &self.seed_to_soil);
        seed = self.adjust(seed, &self.soil_to_fertilizer);
        seed = self.adjust(seed, &self.fertilizer_to_water);
        seed = self.adjust(seed, &self.water_to_light);
        seed = self.adjust(seed, &self.light_to_temperature);
        seed = self.adjust(seed, &self.temperature_to_humidity);
        seed = self.adjust(seed, &self.humidity_to_location);
        seed
    }

    fn adjust(&self, value: u64, adjustments: &RangeMap<u64, i64>) -> u64 {
        let adjustment = adjustments.get(&value).unwrap_or(&0);
        value.checked_add_signed(*adjustment).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Almanac {
        Almanac::parse("
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        ")
    }

    #[test]
    fn test_seed_to_location() {
        let almanac = setup();
        assert_eq!(almanac.seed_to_location(79), 82);
        assert_eq!(almanac.seed_to_location(14), 43);
        assert_eq!(almanac.seed_to_location(55), 86);
        assert_eq!(almanac.seed_to_location(13), 35);
    }

    #[test]
    fn test_solve_part_1() {
        let almanac = setup();
        assert_eq!(solve_part_1(&almanac), 35);
    }

    #[test]
    fn test_as_ranges() {
        let ranges = as_ranges(&vec![2, 1, 10, 5, 8, 4]);
        assert_eq!(ranges.len(), 2);
    }

    #[test]
    fn test_solve_part_2() {
        let almanac = setup();
        assert_eq!(solve_part_2(&almanac), 46);
    }
}
