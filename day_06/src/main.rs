use std::fs::read_to_string;

fn part_1(text: &str) -> u64 {
    RaceConfig::from_str(text).solve_part_1()
}

fn main() {
    let text_file_string =
        read_to_string("./day_06/input.txt").expect("It was supposed to work");
    let part_1_result = part_1(&text_file_string);
    println!("WHAT IS part_1_result?? {part_1_result}");
    // let part_2_result = part_2(&text_file_string);
    // println!("WHAT IS part_2_result?? {part_2_result}");
    /*
        WHAT IS part_1_result?? 2269432
        WHAT IS part_2_result?? ???
    */
}

fn obliterate_whitespace(text: &str) -> String {
    let mut result = String::from(text.trim());
    while result.contains("  ") {
        result = result.replace("  ", " ");
    }
    result
}

#[derive(Debug, PartialEq)]
struct RaceConfig {
    time: Vec<u64>,
    distance: Vec<u64>,
}

impl RaceConfig {
    fn from_str(text: &str) -> RaceConfig {
        let lines = obliterate_whitespace(text);
        let lines = lines.split_once('\n').unwrap();
        let time = lines
            .0
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        let distance = lines
            .1
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        RaceConfig { time, distance }
    }
    fn count_wins_for_race_index(&self, index: usize) -> u64 {
        let time = self.time[index];
        let distance = self.distance[index];
        let mut test = 1;
        let mut total_wins = 0;
        while test < time {
            let time_remaining = time - test;
            let mut test_distance = 0;
            for _ in 0..time_remaining {
                test_distance += test;
            }
            if test_distance > distance {
                total_wins += 1;
            }
            test += 1;
        }
        total_wins
    }
    fn solve_part_1(&self) -> u64 {
        self.time
            .iter()
            .enumerate()
            .map(|(index, _)| self.count_wins_for_race_index(index))
            .product()
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
        assert_eq!(part_1_result, 288);
    }

    #[test]
    fn parse_0() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let race = RaceConfig::from_str(&text_file_string);
        assert_eq!(
            race,
            RaceConfig {
                time: vec![7, 15, 30],
                distance: vec![9, 40, 200]
            }
        );
    }
    #[test]
    fn race_0() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let race = RaceConfig::from_str(&text_file_string);
        assert_eq!(race.count_wins_for_race_index(0), 4);
    }
    #[test]
    fn race_1() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let race = RaceConfig::from_str(&text_file_string);
        assert_eq!(race.count_wins_for_race_index(1), 8);
    }
    #[test]
    fn race_2() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let race = RaceConfig::from_str(&text_file_string);
        assert_eq!(race.count_wins_for_race_index(2), 9);
    }
}
