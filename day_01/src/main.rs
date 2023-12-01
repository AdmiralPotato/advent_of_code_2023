use std::fs::read_to_string;

fn yolo(a: u8) -> u8 {
    a
}

fn main() {
    let sample_input =
        read_to_string("./day_01/input_sample.txt").expect("It was supposed to work");
    let result = yolo(4);
    println!("WHAT IS result?? {result}");
    println!("WHAT IS sample_input?? {sample_input}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        assert_eq!(yolo(4), 4);
    }
}
