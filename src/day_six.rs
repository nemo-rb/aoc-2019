use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};


#[cfg(test)]
mod test_six {
    use super::{run_1, calculate_orbits, parse_orbits};

    
    #[test]
    fn test_parse_orbits() {
        let test_orbits = [("COM".to_string(), "B".to_string()), ("B".to_string(), "C".to_string()), ("C".to_string(),
                            "D".to_string()), ("D".to_string(), "E".to_string()), ("E".to_string(), "F".to_string()),
                            ("B".to_string(), "G".to_string()), ("G".to_string(), "H".to_string()), ("D".to_string(),
                            "I".to_string()), ("E".to_string(), "J".to_string()), ("J".to_string(), "K".to_string()),
                            ("K".to_string(), "L".to_string())];

        let orbits = parse_orbits(&test_orbits);
        assert_eq!(
            42, calculate_orbits(&orbits)
        );
    }


    #[test]
    fn test_run_1() {
        assert_eq!(
            249_308, run_1()
        )
    }
}


pub fn run_1() -> i64{
    let raw_orbits = read_orbits("input/day_six.txt");
    let orbits = parse_orbits(&raw_orbits.unwrap());
    calculate_orbits(&orbits)
}


fn read_orbits(filename: &str) -> Result<Vec<(String, String)>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut orbits:Vec<(String, String)> = Vec::new();

    for line in reader.lines() {
        let orbit:Vec<String> = line?.split(')')
                                .map(str::to_string)
                                .collect();

        orbits.push((orbit[0].clone(), orbit[1].clone()));
    }

    Ok(orbits)
}


fn parse_orbits(raw_orbits: &[(String, String)]) -> HashMap<String, Vec<String>> {
    let mut orbits:HashMap<String, Vec<String>> = HashMap::new();

    for orbit in raw_orbits {
        let k = orbit.0.clone();
        let v = orbit.1.clone();

        match orbits.entry(k) {
            Entry::Occupied(o) => o.into_mut().push(v),
            Entry::Vacant(o) => {
                let mut orbs = Vec::new();
                orbs.push(v);
                o.insert(orbs);
            },
        }
    }

    orbits
}


fn calculate_orbits(orbits: &HashMap<String, Vec<String>>) -> i64 {
    _calculate_orbits(&orbits, "COM", 0)
}


fn _calculate_orbits(orbits: &HashMap<String, Vec<String>>, key: &str, total: i64) -> i64 {
    let mut final_total = total;
    if let Some(orbit) = orbits.get(key) {
        for o in orbit {
            final_total += _calculate_orbits(orbits, o, total + 1);
        }
    }
    final_total
}
