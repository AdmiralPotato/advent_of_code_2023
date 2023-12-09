use std::fs::read_to_string;

fn do_line(input: &[i32]) -> i32 {
    input.last().unwrap() + predict_next_value(input)
}

fn part_1(text: &str) -> i32 {
    text.trim()
        .split("\n")
        .map(|line| {
            let f: Vec<i32> = line
                .trim()
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            do_line(&f)
        })
        .sum()
}
fn part_2(text: &str) -> i32 {
    text.trim()
        .split("\n")
        .map(|line| {
            let mut f: Vec<i32> = line
                .trim()
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            f.reverse();
            do_line(&f)
        })
        .sum()
}

fn predict_next_value(numbers: &[i32]) -> i32 {
    let mut last = numbers[0];
    let diffs: Vec<i32> = numbers[1..]
        .iter()
        .map(|x| {
            let diff = x - last;
            last = *x;
            diff
        })
        .collect();
    // println!("{:?}", diffs);
    let we_need_to_go_deeper = !diffs.iter().all(|x| *x == 0);
    if we_need_to_go_deeper {
        let deeper = predict_next_value(&diffs);
        let last_diff = diffs.iter().last().unwrap();
        deeper + last_diff
    } else {
        0
    }
}

fn main() {
    let text_file_string = read_to_string("./day_09/input.txt").unwrap();
    let part_1_result = part_1(&text_file_string);
    println!("WHAT IS part_1_result?? {part_1_result}");
    let part_2_result = part_2(&text_file_string);
    println!("WHAT IS part_2_result?? {part_2_result}");
    /*
        WHAT IS part_1_result?? 1757008019
        WHAT IS part_2_result?? 995
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let part_1_result = part_1(&text_file_string);
        assert_eq!(part_1_result, 114);
    }

    #[test]
    fn predict_a() {
        let input = &[0, 3, 6, 9, 12, 15];
        assert_eq!(do_line(input), 18);
    }
    #[test]
    fn predict_b() {
        let input = &[1, 3, 6, 10, 15, 21];
        assert_eq!(do_line(input), 28);
    }
    #[test]
    fn predict_c() {
        let input = &[10, 13, 16, 21, 30, 45];
        assert_eq!(do_line(input), 68);
    }
}
