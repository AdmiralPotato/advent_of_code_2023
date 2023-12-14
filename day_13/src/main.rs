use std::fs::read_to_string;

fn part_1(text: &str) -> u32 {
    text.trim()
        .split("\n\n")
        .map(|line| Mirror::from_str(line).solve_part_1())
        .sum()
}
fn part_2(text: &str) -> u32 {
    text.trim()
        .split("\n\n")
        .map(|line| Mirror::from_str(line).solve_part_2())
        .sum()
}

fn main() {
    let text_file_string = read_to_string("./day_13/input.txt").unwrap();
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
struct Mirror {
    pattern: String,
}

enum Axis {
    Horizontal,
    Vertical,
}
impl Mirror {
    fn from_str(line: &str) -> Mirror {
        Mirror {
            pattern: line.trim().to_string(),
        }
    }
    fn transpose(&mut self) {
        let lines: Vec<&str> = self.pattern.trim().split('\n').collect();
        let first_line = lines.first().unwrap();
        let mut new_rows: Vec<String> = vec![];
        for (index, char) in first_line.chars().enumerate() {
            let mut row_so_far = String::new();
            row_so_far += new_rows.get(index).unwrap_or(&"".to_string());
            row_so_far.push(char);
            new_rows.insert(index, row_so_far);
        }
        self.pattern = new_rows.join("\n").trim().to_string();
        println!("{}", self.pattern);
    }

    fn find_symmetry(&mut self) -> (Axis, u32) {
        (Axis::Horizontal, 0)
    }
    fn solve_part_1(&mut self) -> u32 {
        self.transpose();
        0
    }
    fn solve_part_2(&mut self) -> u32 {
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
    fn transpose_a() {
        assert_eq!(
            Mirror::from_str(
                "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            )
            .pattern,
            "
#.##..#
..##...
##..###
#....#.
.#..#.#
.#..#.#
#....#.
##..###
..##..."
                .trim()
        );
    }
    #[test]
    fn solve_a() {
        assert_eq!(
            Mirror::from_str(
                "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            )
            .solve_part_1(),
            10
        );
    }
}
