fn yolo (a: u8) -> u8 {
    a
}

fn main() {
    let result = yolo(4);
    println!("WHAT IS NUMBER?? {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        assert_eq!(yolo(4), 4);
    }
}
