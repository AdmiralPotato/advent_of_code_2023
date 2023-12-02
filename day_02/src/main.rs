use std::fs::read_to_string;
use std::str::Split;

#[derive(Default, Debug, PartialEq)]
struct Cubes {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl Cubes {
    fn from_str(text: &str) -> Cubes {
        let mut result = Cubes::default();
        for color_fragment in text.trim().split(", ") {
            let mut color_fragment = color_fragment.split(" ");
            let count: u8 = color_fragment
                .next()
                .unwrap()
                .parse()
                .expect("unable to parse ");
            let color = color_fragment.next().unwrap();
            match color {
                "red" => result.red = count,
                "green" => result.green = count,
                "blue" => result.blue = count,
                _ => panic!("BAD COLOR LABEL!! {color}"),
            }
        }
        result
    }
    fn test(&self, limit: Cubes) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }
}

#[derive(Debug, PartialEq)]
struct ParsedLine {
    pub id: u8,
    pub handfulls: Vec<Cubes>,
}

const PART_1_CONFIG: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

fn parse_line(text: &str) -> ParsedLine {
    let mut splits = text.split(": ");
    let mut id_fragment = splits.next().unwrap().split(" ");
    id_fragment.next();
    let id = id_fragment
        .next()
        .unwrap()
        .parse()
        .expect("Unable to parse ID");
    let handfulls = splits
        .next()
        .unwrap()
        .split("; ")
        .map(Cubes::from_str)
        .collect();
    ParsedLine { id, handfulls }
}
fn test_line_part_1(text: &str) -> u8 {
    let parsed = parse_line(text);
    if parsed
        .handfulls
        .iter()
        .all(|cubes| cubes.test(PART_1_CONFIG))
    {
        parsed.id
    } else {
        0
    }
}

/*
fn test_line_part_2(text: &str) -> u8 {
    todo!("Implement me later")
}
*/

fn part_1(lines: &Split<&str>) -> u32 {
    lines
        .clone()
        .map(|line| test_line_part_1(line) as u32)
        .sum()
}

/*
fn part_2(lines: &Split<&str>) -> u32 {
    lines
        .clone()
        .map(|line| test_line_part_2(line) as u32)
        .sum()
}
*/

fn main() {
    let text_file_string = read_to_string("./day_02/input.txt").expect("It was supposed to work");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_result = part_1(&sample_input);
    println!("WHAT IS part_1_result?? {part_1_result}");
    // let part_2_result = part_2(sample_input.clone());
    // println!("WHAT IS part_2_result?? {part_2_result}");
    /*
        WHAT IS part_1_result?? 2716
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        // Why are the test run in a different folder? ¯\_(ツ)_/¯
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split("\n");
        let part_1_result = part_1(&sample_input);
        assert_eq!(part_1_result, 8);
    }
    #[test]
    fn parse_0() {
        let sample_a: (&str, ParsedLine) = (
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            ParsedLine {
                id: 1,
                handfulls: vec![
                    Cubes {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Cubes {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Cubes {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
        );
        assert_eq!(parse_line(sample_a.0), sample_a.1);
    }
    #[test]
    fn part_1_test_pass() {
        assert_eq!(
            test_line_part_1("Game 42: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            42
        );
    }
    #[test]
    fn part_1_test_fail() {
        assert_eq!(
            test_line_part_1(
                "Game 17:  8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            0
        );
    }
    // #[test]
    // fn part_2_test_0() {
    //     assert_eq!(parse_line_part_2(SAMPLE_INPUT_B[0]), 29);
    // }
}
