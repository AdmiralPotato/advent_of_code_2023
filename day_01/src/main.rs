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
        println!("Input: {line}, parsed: {parsed}");
        total += parsed;
    }
    total
}

fn main() {
    let text_file_string = read_to_string("./day_01/input.txt").expect("It was supposed to work");
    let sample_input = text_file_string.split("\n");
    let part_1_result = part_1(&sample_input);
    // let result = parse_line(4);
    // println!("WHAT IS result?? {result}");
    println!("WHAT IS part_1_result?? {part_1_result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &[&str] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

    #[test]
    fn part_1_test() {
        // Why are the test run in a different folder? ¯\_(ツ)_/¯
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split("\n");
        let part_1_result = part_1(&sample_input);
        assert_eq!(part_1_result, 142);
    }
    #[test]
    fn test_0() {
        assert_eq!(parse_line_part_1(SAMPLE_INPUT[0]), 12);
    }
    #[test]
    fn test_1() {
        assert_eq!(parse_line_part_1(SAMPLE_INPUT[1]), 38);
    }
    #[test]
    fn test_2() {
        assert_eq!(parse_line_part_1(SAMPLE_INPUT[2]), 15);
    }
    #[test]
    fn test_3() {
        assert_eq!(parse_line_part_1(SAMPLE_INPUT[3]), 77);
    }
}
