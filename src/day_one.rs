use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

use math::round;


#[cfg(test)]
mod calculate_tests {
    use super::{calculate_fuel, calculate_module_fuel};

    #[test]
    fn test_fuel_mass_12() {
        assert_eq!(
            2, calculate_fuel(12)
        );
    }

    #[test]
    fn test_fuel_mass_14() {
        assert_eq!(
            2, calculate_fuel(14)
        );
    }

    #[test]
    fn test_fuel_mass_1969() {
        assert_eq!(
            654, calculate_fuel(1969)
        );
    }

    #[test]
    fn test_fuel_mass_100756() {
        assert_eq!(
            33583, calculate_fuel(100756)
        );
    }

    #[test]
    fn test_fuel_module_12() {
        assert_eq!(
            2, calculate_module_fuel(12)
        );
    }

    #[test]
    fn test_fuel_module_14() {
        assert_eq!(
            2, calculate_module_fuel(14)
        );
    }

    #[test]
    fn test_fuel_module_1969() {
        assert_eq!(
            966, calculate_module_fuel(1969)
        );
    }

    #[test]
    fn test_fuel_module_100756() {
        assert_eq!(
            50346, calculate_module_fuel(100756)
        );
    }
}


fn read_masses(filename: &str) -> Result<Vec<i64>, Error> {
    let file = File::open(filename)?;
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


fn calculate_fuel(mass: i64) -> i64 {
    let _mass = mass as f64;
    let fuel = (round::floor(_mass / 3.0, 0) - 2.0) as i64;

    if fuel <= 0 {
        return 0;
    }

    fuel
}


fn calculate_module_fuel(mass: i64) -> i64 {
    if mass <= 0 {
        return 0;
    }

    let fuel = calculate_fuel(mass);
    fuel + calculate_module_fuel(fuel)
}


pub fn run() -> Result<i64, Error> {
    let filename = "input/day_one.txt";
    let masses = read_masses(&filename)?;
    let mut fuel = 0;

    for mass in masses {
        fuel += calculate_module_fuel(mass);
    }
    Ok(fuel)
}
