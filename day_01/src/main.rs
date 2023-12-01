use std::fs::read_to_string;
use std::str::Split;

fn parse_line_part_1(text: &str) -> u8 {
    let mut result = String::new();
    let mut last_char = ' ';
    let mut count = 0;
    for char in text.chars() {
        let is_digit = matches!(
            char,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
        );
        if is_digit {
            last_char = char;
            count += 1;
            if count == 1 {
                result += &format!("{last_char}");
            }
        }
    }
    if result.len() == 1 {
        result += &format!("{last_char}");
    }
    let oof = format!("What is combined? {result}");
    (*result).parse::<u8>().expect(&oof)
}
fn part_1(lines: &Split<&str>) -> u32 {
    let mut total = 0;
    for line in lines.clone() {
        if line.is_empty() {
            continue;
        }
        let parsed = parse_line_part_1(line) as u32;
        // println!("Input: {line}, parsed: {parsed}");
        total += parsed;
    }
    total
}

const STRINGS_TO_NUMBERS: [(&str, char); 10] = [
    ("zero", '0'),
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];
fn parse_line_part_2(text: &str) -> u8 {
    let mut result = String::new();
    let mut last_char = ' ';
    let mut count = 0;
    for (char_index, mut char) in text.chars().enumerate() {
        let mut is_digit = matches!(
            char,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
        );
        if !is_digit {
            for (word, char_value) in STRINGS_TO_NUMBERS {
                if text[char_index..].starts_with(word) {
                    is_digit = true;
                    char = char_value;
                }
            }
        }
        if is_digit {
            last_char = char;
            count += 1;
            if count == 1 {
                result += &format!("{last_char}");
            }
        }
    }
    if result.len() == 1 {
        result += &format!("{last_char}");
    }
    let oof = format!("What is combined? {result}");
    (*result).parse::<u8>().expect(&oof)
}
fn part_2(lines: &Split<&str>) -> u32 {
    let mut total = 0;
    for line in lines.clone() {
        if line.is_empty() {
            continue;
        }
        let parsed = parse_line_part_2(line) as u32;
        // println!("Input: {line}, parsed: {parsed}");
        total += parsed;
    }
    total
}

fn main() {
    let text_file_string = read_to_string("./day_01/input.txt").expect("It was supposed to work");
    let sample_input = text_file_string.split("\n");
    let part_1_result = part_1(&sample_input);
    println!("WHAT IS part_1_result?? {part_1_result}");
    let part_2_result = part_2(&sample_input);
    println!("WHAT IS part_2_result?? {part_2_result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT_A: &[&str] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    const SAMPLE_INPUT_B: &[&str] = &[
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    #[test]
    fn part_1_test() {
        // Why are the test run in a different folder? ¯\_(ツ)_/¯
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split("\n");
        let part_1_result = part_1(&sample_input);
        assert_eq!(part_1_result, 142);
    }
    #[test]
    fn part_1_test_0() {
        assert_eq!(parse_line_part_1(SAMPLE_INPUT_A[0]), 12);
    }
    #[test]
    fn part_1_test_1() {
        assert_eq!(parse_line_part_1(SAMPLE_INPUT_A[1]), 38);
    }
    #[test]
    fn part_1_test_2() {
        assert_eq!(parse_line_part_1(SAMPLE_INPUT_A[2]), 15);
    }
    #[test]
    fn part_1_test_3() {
        assert_eq!(parse_line_part_1(SAMPLE_INPUT_A[3]), 77);
    }
    #[test]
    fn part_2_test() {
        let text_file_string = read_to_string("./input_sample_2.txt").unwrap();
        let sample_input = text_file_string.trim().split("\n");
        let part_2_result = part_2(&sample_input);
        assert_eq!(part_2_result, 281);
    }
    #[test]
    fn part_2_test_0() {
        assert_eq!(parse_line_part_2(SAMPLE_INPUT_B[0]), 29);
    }
    #[test]
    fn part_2_test_1() {
        assert_eq!(parse_line_part_2(SAMPLE_INPUT_B[1]), 83);
    }
    #[test]
    fn part_2_test_2() {
        assert_eq!(parse_line_part_2(SAMPLE_INPUT_B[2]), 13);
    }
    #[test]
    fn part_2_test_3() {
        assert_eq!(parse_line_part_2(SAMPLE_INPUT_B[3]), 24);
    }
    #[test]
    fn part_2_test_4() {
        assert_eq!(parse_line_part_2(SAMPLE_INPUT_B[4]), 42);
    }
    #[test]
    fn part_2_test_5() {
        assert_eq!(parse_line_part_2(SAMPLE_INPUT_B[5]), 14);
    }
    #[test]
    fn part_2_test_6() {
        assert_eq!(parse_line_part_2(SAMPLE_INPUT_B[6]), 76);
    }
}
