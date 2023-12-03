use regex::Regex;
use std::cmp;
use std::fs::read_to_string;
use std::ops::Range;

fn parse_file(input: &str) -> Vec<&str> {
    input.trim().split('\n').map(|line| line.trim()).collect()
}

fn get_rect(x: Range<usize>, y: usize, inputs: &Vec<&str>) -> String {
    let y_min = cmp::max(y.saturating_sub(1), 0);
    let y_max = cmp::min(y + 1, inputs.len() - 1);
    let x_min = cmp::max(x.start.saturating_sub(1), 0);
    let x_max = cmp::min(x.end, inputs[y].len() - 1);
    // let yange = y_min..=y_max;
    // let xange = x_min..=x_max;
    // println!("yange: {:#?} || xange: {:#?}", yange, xange);
    let vec_of_str: Vec<&str> =
        (y_min..=y_max).map(|y| &inputs[y][x_min..=x_max]).collect();
    vec_of_str.join("\n")
}

const SYMBOLS: [&str; 10] = ["#", "$", "%", "&", "*", "+", "-", "/", "=", "@"];
fn test_part(text: &str) -> bool {
    SYMBOLS.iter().any(|symbol| text.contains(symbol))
}

fn search_line(y: usize, line: &str, parsed: &Vec<&str>) -> Vec<u32> {
    Regex::new(r"(\d+)")
        .unwrap()
        .captures_iter(line)
        .map(|captures| {
            let single_match = captures.get(0).unwrap();
            // println!("what got captured? {:#?}", single_match);
            // println!("what got range? {:#?}", single_match.range());
            // println!("what got str? {}", single_match.as_str());
            let range = single_match.range();
            let rect = get_rect(range.clone(), y, parsed);
            (
                range,
                single_match
                    .as_str()
                    .parse::<u32>()
                    .expect("Parse number bad go boom"),
                rect.clone(),
                test_part(&rect),
            )
        })
        .filter(|(_, _, _, passed)| *passed)
        .map(|(_, value, _, _)| value)
        .collect()
}

fn part_1(text: &str) -> u32 {
    let parsed = parse_file(text);
    parsed
        .iter()
        .enumerate()
        .map(|(y, line)| {
            search_line(y, line, &parsed).into_iter().sum::<u32>()
        })
        .sum()
}

fn main() {
    let text_file_string =
        read_to_string("./day_03/input.txt").expect("It was supposed to work");
    let part_1_result = part_1(&text_file_string);
    println!("WHAT IS part_1_result?? {part_1_result}");
    // let part_2_result = part_2(&text_file_string);
    // println!("WHAT IS part_2_result?? {part_2_result}");
    /*
        WHAT IS part_1_result?? 553079
        WHAT IS part_2_result?? ???
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        // Why are the test run in a different folder? ¯\_(ツ)_/¯
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let part_1_result = part_1(&text_file_string);
        assert_eq!(part_1_result, 4361);
    }
    #[test]
    fn parse_0() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let part_1_result = parse_file(&text_file_string);
        assert_eq!(part_1_result.len(), 10);
        assert_eq!(part_1_result[0].len(), 10);
    }
    #[test]
    fn box_a() {
        let text = "
........
....*...
....42..
........
        ";
        let expectation = "
.*..
.42.
....
        "
        .trim();
        let parsed = parse_file(text);
        assert_eq!(
            parsed,
            vec!["........", "....*...", "....42..", "........",],
            "parse failure"
        );
        let rect = get_rect(4..6, 2, &parsed);
        assert_eq!(rect, expectation, "get_rect failure");
    }
    #[test]
    fn box_b() {
        let text = "
123.....
..*.....
........
........
        ";
        let expectation = "
123
..*
        "
        .trim();
        let parsed = parse_file(text);
        let rect = get_rect(0..2, 0, &parsed);
        assert_eq!(rect, expectation, "get_rect failure");
    }
    #[test]
    fn box_c() {
        let text = "
........
........
.......*
.....789
        ";
        let expectation = "
...*
.789
        "
        .trim();
        let parsed = parse_file(text);
        let rect = get_rect(5..8, 3, &parsed);
        assert_eq!(rect, expectation, "get_rect failure");
    }
    #[test]
    fn part_yes() {
        let items = [
            "...#\n789",
            "...$\n789",
            "...%\n789",
            "...&\n789",
            "...*\n789",
            "...+\n789",
            "...-\n789",
            ".../\n789",
            "...=\n789",
            "...@\n789",
        ];
        for text in items {
            assert!(test_part(text), "test_part failure");
        }
    }
    #[test]
    fn part_no() {
        assert!(!test_part("...\n789\n..."), "test_part failure");
    }
}
