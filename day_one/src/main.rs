use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("input/calibration_document.txt");
    let lines = read_lines(path).expect("Could not read file");

    let mut calibration_sum: i32 = 0;

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        let calibration_line = line.expect("Could not read line");
        let mut next_number: String = "".to_string();
        let mut end_digit: String = "".to_string();
        for char in calibration_line.chars() {
            // convert char to i32
            if char.is_ascii_digit() {
                if next_number.is_empty() {
                    next_number += &char.to_string();
                    end_digit = char.to_string();
                } else {
                    end_digit = char.to_string();
                }
            }
        }
        next_number += &end_digit;
        let increment: i32 = next_number.parse::<i32>().unwrap();
        calibration_sum += increment;
    }

    print!("Calibration sum: {}", calibration_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
