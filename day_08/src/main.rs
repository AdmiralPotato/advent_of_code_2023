mod paths;
use paths::*;
use std::collections::HashMap;

fn part_1() -> u64 {
    get_steps(PATH_REAL, MAP_REAL)
}
fn part_2() -> u64 {
    0
}

fn get_steps(path: &str, map: &[(&str, (&str, &str))]) -> u64 {
    let mut chars_cycle = path.chars().cycle();
    let mappy: HashMap<&str, (&str, &str)> = map.iter().copied().collect();
    let mut noot = String::from("AAA");
    let mut steps = 0;
    while noot != "ZZZ" {
        let next_char = chars_cycle.next().unwrap();
        let current_position = mappy.get(noot.as_str()).unwrap();
        noot = match next_char {
            'L' => current_position.0.to_string(),
            _ => current_position.1.to_string(),
        };
        steps += 1;
    }
    steps
}

fn main() {
    let part_1_result = part_1();
    println!("WHAT IS part_1_result?? {part_1_result}");
    let part_2_result = part_2();
    println!("WHAT IS part_2_result?? {part_2_result}");
    /*
        WHAT IS part_1_result?? ???
        WHAT IS part_2_result?? ???
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_a() {
        let part_1_result = get_steps(PATH_SAMPLE_A, MAP_SAMPLE_A);
        assert_eq!(part_1_result, 2);
    }

    #[test]
    fn part_1_test_b() {
        let part_1_result = get_steps(PATH_SAMPLE_B, MAP_SAMPLE_B);
        assert_eq!(part_1_result, 6);
    }
}
