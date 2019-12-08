use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

use math::round;


#[cfg(test)]
mod calculate_tests {
    use super::calculate;

    #[test]
    fn test_12() {
        assert_eq!(
            2, calculate(12)
        );
    }

    #[test]
    fn test_14() {
        assert_eq!(
            2, calculate(14)
        );
    }

    #[test]
    fn test_1969() {
        assert_eq!(
            654, calculate(1969)
        );
    }

    #[test]
    fn test_100756() {
        assert_eq!(
            33583, calculate(100756)
        );
    }
}


fn read_masses(filename: &str) -> Result<Vec<i64>, Error> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut masses = Vec::new();

    for line in reader.lines() {
        masses.push(line?
            .trim()
            .parse::<i64>()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(masses)
}


fn calculate(mass: i64) -> i64 {
    let _mass = mass as f64;
    (round::floor(_mass / 3.0, 0) - 2.0) as i64
}


pub fn calculate_fuel_required() -> Result<i64, Error> {
    let filename = "input/day_one.txt";
    let masses = read_masses(&filename).unwrap();
    let mut fuel = 0;

    for mass in masses {
        fuel += calculate(mass);
    }
    Ok(fuel)
}
