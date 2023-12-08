use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::mpsc::channel;

fn part_1(text: &str) -> u64 {
    0
}
fn part_2(text: &str) -> u64 {
    0
}

fn main() {
    let text_file_string =
        read_to_string("./day_06/input.txt").expect("It was supposed to work");
    let part_1_result = part_1(&text_file_string);
    println!("WHAT IS part_1_result?? {part_1_result}");
    let part_2_result = part_2(&text_file_string);
    println!("WHAT IS part_2_result?? {part_2_result}");
    /*
        WHAT IS part_1_result?? 2269432
        WHAT IS part_2_result?? ???
    */
}

#[derive(Debug, PartialEq)]
enum Classification {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
const VALUES: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn classify(text: &str) -> Classification {
    let mut map: HashMap<char, u32> = HashMap::new();
    for i in text.split_once(' ').unwrap().0.chars() {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut values: Vec<u32> = map.values().copied().collect();
    values.sort();
    values.reverse();
    println!("what the map {:?}", map);
    println!("what the sort {:?}", values);
    match values {
        values[0] == 5 => Classification::HighCard,
        _ => Classification::HighCard,
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
        assert_eq!(part_1_result, 6440);
    }

    #[test]
    fn classify_0() {
        let oof = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
        "
        .trim()
        .split('\n');
        let meow: Vec<Classification> = oof.map(classify).collect();
        assert_eq!(meow, vec![Classification::OnePair,]);
    }

    #[test]
    fn classify_1() {
        let oof = "
12345 111
11234 111
11223 111
11123 111
11122 111
11112 111
11111 111
        "
        .trim()
        .split('\n');
        let meow: Vec<Classification> = oof.map(classify).collect();
        assert_eq!(
            meow,
            vec![
                Classification::HighCard,
                Classification::OnePair,
                Classification::TwoPair,
                Classification::ThreeOfAKind,
                Classification::FullHouse,
                Classification::FourOfAKind,
                Classification::FiveOfAKind,
            ]
        );
    }
}
