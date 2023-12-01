use std::fs::read_to_string;
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
    (*result).parse::<u8>().expect("cannot parse u8")
}

fn main() {
    let sample_input = read_to_string("./day_01/input.txt").expect("It was supposed to work");
    // let result = parse_line(4);
    // println!("WHAT IS result?? {result}");
    println!("WHAT IS sample_input?? {sample_input}");
}

const SAMPLE_INPUT: &[&str] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

#[cfg(test)]
mod tests {
    use super::*;

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
