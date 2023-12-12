use std::fs::read_to_string;

fn part_1(text: &str) -> u32 {
    text.trim()
        .split('\n')
        .map(|line| SpringTime::from_str(line).solve_part_1())
        .sum()
}
fn part_2(text: &str) -> i32 {
    0
}

fn main() {
    let text_file_string = read_to_string("./day_12/input.txt").unwrap();
    let part_1_result = part_1(&text_file_string);
    println!("WHAT IS part_1_result?? {part_1_result}");
    let part_2_result = part_2(&text_file_string);
    println!("WHAT IS part_2_result?? {part_2_result}");
    /*
        WHAT IS part_1_result?? ???
        WHAT IS part_2_result?? ???
    */
}

#[derive(Debug, PartialEq)]
struct SpringTime {
    pattern: String,
    rules: Vec<u32>,
}

impl SpringTime {
    fn from_str(line: &str) -> SpringTime {
        let (pattern, rules) = line.trim().split_once(' ').unwrap();
        let pattern = pattern.to_string();
        let rules = rules.split(',').map(|x| x.parse().unwrap()).collect();
        SpringTime { pattern, rules }
    }

    fn solve_part_1(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let part_1_result = part_1(&text_file_string);
        assert_eq!(part_1_result, 21);
    }
    #[test]
    fn parse() {
        let tests = [
            (
                "???.### 1,1,3",
                SpringTime {
                    pattern: "???.###".to_string(),
                    rules: vec![1, 1, 3],
                },
            ),
            (
                "?#?#?#?#?#?#?#? 1,3,1,6",
                SpringTime {
                    pattern: "?#?#?#?#?#?#?#?".to_string(),
                    rules: vec![1, 3, 1, 6],
                },
            ),
        ];
        for (input, expectation) in tests {
            assert_eq!(SpringTime::from_str(input), expectation);
        }
    }
    #[test]
    fn solve_a() {
        assert_eq!(SpringTime::from_str("???.### 1,1,3").solve_part_1(), 1);
    }
    #[test]
    fn solve_b() {
        assert_eq!(
            SpringTime::from_str(".??..??...?##. 1,1,3").solve_part_1(),
            4
        );
    }
    #[test]
    fn solve_c() {
        assert_eq!(
            SpringTime::from_str("?#?#?#?#?#?#?#? 1,3,1,6").solve_part_1(),
            1
        );
    }
    #[test]
    fn solve_d() {
        assert_eq!(
            SpringTime::from_str("????.#...#... 4,1,1").solve_part_1(),
            1
        );
    }
    #[test]
    fn solve_e() {
        assert_eq!(
            SpringTime::from_str("????.######..#####. 1,6,5").solve_part_1(),
            4
        );
    }
    #[test]
    fn solve_f() {
        assert_eq!(
            SpringTime::from_str("?###???????? 3,2,1").solve_part_1(),
            10
        );
    }
}
