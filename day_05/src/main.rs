use std::fs::read_to_string;
use std::ops::Range;

fn part_1(text: &str) -> u64 {
    let mut mapped_numbers: Vec<u64> = text
        .trim()
        .split('\n')
        .map(|line| WholePuzzle::from_str(line.trim()).solve_part_1())
        .collect();
    mapped_numbers.sort();
    mapped_numbers[0]
}

fn main() {
    let text_file_string =
        read_to_string("./day_05/input.txt").expect("It was supposed to work");
    let part_1_result = part_1(&text_file_string);
    println!("WHAT IS part_1_result?? {part_1_result}");
    // let part_2_result = part_2(&text_file_string);
    // println!("WHAT IS part_2_result?? {part_2_result}");
    /*
        WHAT IS part_1_result?? ???
        WHAT IS part_2_result?? ???
    */
}

#[derive(Debug, PartialEq)]
struct Ranges {
    dest: Range<u64>,
    start: Range<u64>,
}
#[derive(Debug, PartialEq)]
struct Map {
    label: String,
    map: Vec<Ranges>,
}
impl Map {
    fn from_str(text: &str) -> Map {
        let mut lines = text.trim().split('\n');
        let label =
            lines.next().unwrap().split_once(' ').unwrap().0.to_string();
        let ranges: Vec<Ranges> = lines
            .map(|line| {
                let ranges: Vec<u64> = line
                    .split(' ')
                    .map(|x| {
                        x.parse().unwrap_or_else(|_| {
                            panic!("Bad parse range number! {x}")
                        })
                    })
                    .collect();
                let range = ranges[2] - 1;
                let output = ranges[0];
                let output_end = output + range;
                let input = ranges[1];
                let input_end = input + range;
                Ranges {
                    dest: output..output_end,
                    start: input..input_end,
                }
            })
            .collect();
        Map { label, map: ranges }
    }
    fn map_seed(&self, seed: u64) -> u64 {
        0
    }
}
#[derive(Debug, PartialEq)]
struct WholePuzzle {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}
impl WholePuzzle {
    fn from_str(text: &str) -> WholePuzzle {
        let mut lines = text.trim().split("\n\n");
        let seeds = WholePuzzle::get_seeds(lines.next().unwrap());
        let mut maps: Vec<Map> = vec![];
        lines.for_each(|section| {
            maps.push(Map::from_str(section));
        });
        WholePuzzle { seeds, maps }
    }
    fn get_seeds(text: &str) -> Vec<u64> {
        text.trim()[7..]
            .split(' ')
            .map(|x| {
                x.parse()
                    .unwrap_or_else(|_| panic!("Bad parse number! {x}"))
            })
            .collect()
    }
    fn put_seed_through_all_maps(
        seed: u64,
        ranges: Vec<(Range<u64>, Range<u64>)>,
    ) -> u64 {
        0
    }
    fn solve_part_1(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    impl From<(&str, Vec<(Range<u64>, Range<u64>)>)> for Map {
        fn from(value: (&str, Vec<(Range<u64>, Range<u64>)>)) -> Self {
            let map = value
                .1
                .into_iter()
                .map(|(dest, start)| Ranges {
                    dest: dest.clone(),
                    start: start.clone(),
                })
                .collect();
            Map {
                label: value.0.to_string(),
                map,
            } // I'm the map!
        }
    }
    #[test]
    fn part_1_test() {
        // Why are the test run in a different folder? ¯\_(ツ)_/¯
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let part_1_result = part_1(&text_file_string);
        assert_eq!(part_1_result, 35);
    }
    const SIMPLE_MAP_TEST: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48
        ";
    #[test]
    fn parse_0() {
        assert_eq!(
            WholePuzzle::from_str(SIMPLE_MAP_TEST),
            WholePuzzle {
                seeds: vec![79, 14, 55, 13],
                maps: vec![Map {
                    label: "seed-to-soil".to_string(),
                    map: vec![
                        Ranges {
                            dest: 50..51,
                            start: 98..99,
                        },
                        Ranges {
                            dest: 52..99,
                            start: 50..97,
                        },
                    ],
                }]
            }
        );
    }
    #[test]
    fn map_0() {
        assert_eq!(WholePuzzle::from_str(SIMPLE_MAP_TEST).solve_part_1(), 13);
    }
    #[test]
    fn parse_seeds_0() {
        let input = "seeds: 79 14 55 13";
        assert_eq!(WholePuzzle::get_seeds(input), vec![79, 14, 55, 13]);
    }
    #[test]
    fn parse_map_0() {
        let input = "
fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4
        ";
        assert_eq!(
            Map::from_str(input),
            (
                "fertilizer-to-water",
                vec![
                    (49..56, 53..60),
                    (0..41, 11..52),
                    (42..48, 0..6),
                    (57..60, 7..10),
                ]
            )
                .into()
        );
    }
}
