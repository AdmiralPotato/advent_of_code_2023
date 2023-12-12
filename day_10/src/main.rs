use std::fs::read_to_string;

fn find_start_position(lines: &[Vec<Cell>]) -> (u32, u32) {
    for (y, line) in lines.iter().enumerate() {
        if let Some(x) = line.iter().position(|c| *c == Cell::Snorlax) {
            return (x as u32, y as u32);
        }
    }
    (0, 0)
}

#[derive(Copy, Clone, Default, PartialEq)]
enum Cell {
    #[default]
    None,
    NorthSouth,
    NorthEast,
    NorthWest,
    EastSouth,
    EastWest,
    SouthWest,
    Snorlax,
}
impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '|' => Cell::NorthSouth,
            '-' => Cell::EastWest,
            'L' => Cell::NorthEast,
            '7' => Cell::SouthWest,
            'J' => Cell::NorthWest,
            'F' => Cell::EastSouth,
            'S' => Cell::Snorlax,
            _ => Cell::None,
        }
    }
    fn connects_north(&self) -> bool {
        matches!(self, Cell::NorthWest | Cell::NorthEast | Cell::NorthSouth)
    }
    fn connects_east(&self) -> bool {
        matches!(self, Cell::NorthEast | Cell::EastWest | Cell::EastSouth)
    }
    fn connects_south(&self) -> bool {
        matches!(self, Cell::NorthSouth | Cell::EastSouth | Cell::SouthWest)
    }
    fn connects_west(&self) -> bool {
        matches!(self, Cell::NorthWest | Cell::EastWest | Cell::SouthWest)
    }
}

fn splitsy_doodle(text: &str) -> Vec<Vec<Cell>> {
    text.trim()
        .split('\n')
        .map(|line| line.chars().map(Cell::from_char).collect())
        .collect()
}
#[derive(Debug)]
struct NeighborMap {
    n: bool,
    e: bool,
    s: bool,
    w: bool,
    n_pos: (u32, u32),
    e_pos: (u32, u32),
    s_pos: (u32, u32),
    w_pos: (u32, u32),
}
fn get_value_at_position(position: (u32, u32), lines: &[Vec<Cell>]) -> Cell {
    lines[position.1 as usize][position.0 as usize]
}
fn get_valid_neighbors(
    position: (u32, u32),
    lines: &[Vec<Cell>],
) -> NeighborMap {
    let n_pos = (position.0, position.1 - 1);
    let e_pos = (position.0 + 1, position.1);
    let s_pos = (position.0, position.1 + 1);
    let w_pos = (position.0 - 1, position.1);
    let cell = get_value_at_position(position, lines);
    let n;
    let e;
    let s;
    let w;
    if cell == Cell::Snorlax {
        let cell = get_value_at_position(n_pos, lines);
        n = cell.connects_south();
        let cell = get_value_at_position(e_pos, lines);
        e = cell.connects_west();
        let cell = get_value_at_position(s_pos, lines);
        s = cell.connects_north();
        let cell = get_value_at_position(w_pos, lines);
        w = cell.connects_east();
    } else {
        n = cell.connects_north();
        e = cell.connects_east();
        s = cell.connects_south();
        w = cell.connects_west();
    }
    NeighborMap {
        n,
        e,
        s,
        w,
        n_pos,
        e_pos,
        s_pos,
        w_pos,
    }
}
fn go_to_next_point(
    position: (u32, u32),
    lines: &[Vec<Cell>],
    visited: &[(u32, u32)],
) -> (u32, u32) {
    let last_position = visited.iter().last();
    let neighbors = get_valid_neighbors(position, lines);

    if neighbors.n && last_position != Some(&neighbors.n_pos) {
        return neighbors.n_pos;
    } else if neighbors.e && last_position != Some(&neighbors.e_pos) {
        return neighbors.e_pos;
    } else if neighbors.s && last_position != Some(&neighbors.e_pos) {
        return neighbors.s_pos;
    } else if neighbors.w && last_position != Some(&neighbors.w_pos) {
        return neighbors.w_pos;
    }
    panic!("No valid directions to go?");
}
fn find_pipe_line_from_start(
    start: (u32, u32),
    lines: &[Vec<Cell>],
) -> Vec<(u32, u32)> {
    let mut current_pos = start;
    let mut visited: Vec<(u32, u32)> = vec![start];
    let mut loop_complete = false;
    let mut steps = 0;
    while !loop_complete {
        println!(
            "find_pipe_line_from_start: current_pos before {:?}",
            current_pos
        );
        current_pos = go_to_next_point(current_pos, lines, &visited);
        visited.push(current_pos);
        println!(
            "find_pipe_line_from_start: current_pos after {:?}",
            current_pos
        );
        println!("find_pipe_line_from_start: visited after {:?}", visited);
        steps += 1;
        if current_pos == start || steps > 5 {
            loop_complete = true;
        }
    }
    visited
}

fn part_1(text: &str) -> i32 {
    let lines = splitsy_doodle(text);
    let start = find_start_position(&lines);

    // text.trim()
    //     .split("\n")
    //     .map(|line| {
    //         let f: Vec<i32> = line
    //             .trim()
    //             .split(' ')
    //             .map(|x| x.parse::<i32>().unwrap())
    //             .collect();
    //     })
    //     .sum()
    0
}
fn part_2(text: &str) -> i32 {
    0
}

fn main() {
    let text_file_string = read_to_string("./day_10/input.txt").unwrap();
    let part_1_result = part_1(&text_file_string);
    println!("WHAT IS part_1_result?? {part_1_result}");
    let part_2_result = part_2(&text_file_string);
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
    fn test_part_1() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let part_1_result = part_1(&text_file_string);
        assert_eq!(part_1_result, 4);
    }

    #[test]
    fn test_part_2() {
        let text_file_string = read_to_string("./input_sample_b.txt").unwrap();
        let part_1_result = part_1(&text_file_string);
        assert_eq!(part_1_result, 8);
    }

    #[test]
    fn find_start_a() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let lines = splitsy_doodle(&text_file_string);
        assert_eq!(find_start_position(&lines), (1, 1));
    }

    #[test]
    fn find_start_b() {
        let text_file_string = read_to_string("./input_sample_b.txt").unwrap();
        let lines = splitsy_doodle(&text_file_string);
        assert_eq!(find_start_position(&lines), (0, 2));
    }

    #[test]
    fn loop_a() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let lines = splitsy_doodle(&text_file_string);
        let start = find_start_position(&lines);
        assert_eq!(
            find_pipe_line_from_start(start, &lines),
            [
                (1, 1),
                (2, 1),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (1, 3),
                (1, 2),
            ]
        );
    }

    #[test]
    fn loop_b() {
        let text_file_string = read_to_string("./input_sample_b.txt").unwrap();
        let lines = splitsy_doodle(&text_file_string);
        let start = find_start_position(&lines);
        assert_eq!(
            find_pipe_line_from_start(start, &lines),
            [
                (0, 2),
                (1, 2),
                (1, 1),
                (2, 1),
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 3),
                (2, 3),
                (1, 3),
                (1, 4),
                (0, 4),
                (0, 3),
            ]
        );
    }
}
