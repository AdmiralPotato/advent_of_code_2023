use std::fs::read_to_string;

fn part_1(text: &str) -> u32 {
    text.trim()
        .split('\n')
        .map(|line| Card::from_str(line.trim()).score)
        .sum()
}

fn main() {
    let text_file_string =
        read_to_string("./day_04/input.txt").expect("It was supposed to work");
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
struct Card {
    id: u32,
    winning: Vec<u32>,
    cells: Vec<u32>,
    count: u32,
    score: u32,
}
impl Card {
    fn from_str(text: &str) -> Card {
        let text = text
            .replace("  ", " ")
            .replace("  ", " ")
            .replace("  ", " ")
            .replace("  ", " ");
        let (id_fragment, rest) =
            text.trim().split_once(": ").expect("Bad line");
        let id = id_fragment[5..]
            .parse()
            .unwrap_or_else(|_| panic!("Bad id {id_fragment}"));
        let (winning_str, cells_str) =
            rest.split_once(" | ").expect("need pipe");
        let winning = Card::get_numbers(winning_str);
        let cells = Card::get_numbers(cells_str);
        let count = cells
            .iter()
            .filter(|x| winning.contains(x))
            .collect::<Vec<&u32>>()
            .len() as u32;
        let score = if count > 0 {
            2u32.pow(count.saturating_sub(1))
        } else {
            0
        };
        Card {
            id,
            winning,
            cells,
            count,
            score,
        }
    }
    fn get_numbers(text: &str) -> Vec<u32> {
        // println!("get_numbers: What is text? {text}");
        text.trim()
            .split(' ')
            .map(|t| t.parse().expect("Bad number"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        // Why are the test run in a different folder? ¯\_(ツ)_/¯
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let part_1_result = part_1(&text_file_string);
        assert_eq!(part_1_result, 13);
    }
    #[test]
    fn parse_0() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(
            Card::from_str(input),
            Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                cells: vec![83, 86, 6, 31, 17, 9, 48, 53],
                count: 4,
                score: 8
            }
        );
    }
    #[test]
    fn score_part_1() {
        let input = [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 4, 8),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2, 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2, 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1, 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0, 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0, 0),
        ];
        for (string, count_e, score_e) in input {
            let Card { count, score, .. } = Card::from_str(string);
            println!("Testing {string}; score: {score} | {score_e}; count: {count} | {count_e}");
            assert_eq!(count, count_e);
            assert_eq!(score, score_e);
        }
    }
}
