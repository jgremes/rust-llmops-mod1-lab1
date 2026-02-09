/*
This is a calculator script. It has the following features:
* Addition
* Subtraction
* Multiplication
* Division
* Power
*/

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

pub fn power(a: i32, b: i32) -> i32 {
    a.pow(b as u32)
}

//test code here with unit test macro
#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::PathBuf;

    fn read_a_b_r(filename: &str) -> (i32, i32, i32) {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        path.push("tests/fixtures");

        path.push(filename);

        let file = File::open(path).expect("El txt no esta");

        let mut lines = io::BufReader::new(file).lines();

        let a: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        let b: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        let c: i32 = lines.next().unwrap().unwrap().parse().unwrap();

        (a, b, c)
    }

    #[test]
    fn test_add() {
        let (a, b, r) = read_a_b_r("data_add.txt");
        assert_eq!(add(a, b), r);
    }
    #[test]
    fn test_subtract() {
        let (a, b, r) = read_a_b_r("data_substract.txt");
        assert_eq!(subtract(a, b), r);
    }
    #[test]
    fn test_multiply() {
        let (a, b, r) = read_a_b_r("data_multiply.txt");
        assert_eq!(multiply(a, b), r);
    }
    #[test]
    fn test_divide() {
        let (a, b, r) = read_a_b_r("data_divide.txt");
        assert_eq!(divide(a, b), r);
    }
    #[test]
    fn test_power() {
        let (a, b, r) = read_a_b_r("data_power.txt");
        assert_eq!(power(a, b), r);
    }
}
