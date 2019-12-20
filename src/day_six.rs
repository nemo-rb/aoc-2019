use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use itertools::Itertools;


#[cfg(test)]
mod test_six {
    use super::{run_1, calculate_orbits, find_orbit, find_lco, Itertools, calculate_orbital_transfers, run_2};


    #[test]
    fn test_calculate_orbits() {
        let orbits = vec![("COM", "B"), ("B", "C"), ("C", "D"), ("D", "E"), ("E", "F"),
                          ("B", "G"), ("G", "H"), ("D", "I"), ("E", "J"), ("J", "K"),
                          ("K", "L")].into_iter().into_group_map();

        assert_eq!(
            42, calculate_orbits(&orbits)
        );
    }


    #[test]
    fn test_calculate_transfer() {
        let orbits = vec![("COM", "B"), ("B", "C"), ("C", "D"), ("D", "E"), ("E", "F"),
                          ("B", "G"), ("G", "H"), ("D", "I"), ("E", "J"), ("J", "K"),
                          ("K", "L"), ("K", "YOU"), ("I", "SAN")].into_iter().into_group_map();

        assert_eq!(
            6, find_orbit(&orbits, "K")
        );
    }


    #[test]
    fn test_calculate_lco() {
        let orbits = vec![("COM", "B"), ("B", "C"), ("C", "D"), ("D", "E"), ("E", "F"),
                          ("B", "G"), ("G", "H"), ("D", "I"), ("E", "J"), ("J", "K"),
                          ("K", "L"), ("K", "YOU"), ("I", "SAN")].into_iter().into_group_map();

        assert_eq!(
            "D", find_lco(&orbits, "YOU", "SAN")
        );
    }


    #[test]
    fn test_calculate_transfers() {
        let orbits = vec![("COM", "B"), ("B", "C"), ("C", "D"), ("D", "E"), ("E", "F"),
                          ("B", "G"), ("G", "H"), ("D", "I"), ("E", "J"), ("J", "K"),
                          ("K", "L"), ("K", "YOU"), ("I", "SAN")].into_iter().into_group_map();

        assert_eq!(
            4, calculate_orbital_transfers(&orbits)
        );
    }


    #[test]
    fn test_run_1() {
        assert_eq!(
            249_308, run_1()
        )
    }

    #[test]
    fn test_run_2() {
        assert_eq!(
            349, run_2()
        )
    }
}


pub fn run_1() -> i64{
    let mut file = File::open("input/day_six.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let orbits = read_orbits(&mut s);

    calculate_orbits(&orbits.unwrap())
}


pub fn run_2() -> i64{
    let mut file = File::open("input/day_six.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let orbits = read_orbits(&mut s);

    calculate_orbital_transfers(&orbits.unwrap())
}


fn read_orbits(s: &mut String) -> Option<HashMap<&str, Vec<&str>>> {
    let orbits = s.lines()
                  .map(|line| line.split(')').collect_tuple().unwrap())
                  .into_group_map();

    Some(orbits.clone())
}


fn calculate_orbits(orbits: &HashMap<&str, Vec<&str>>) -> i64 {
    _calculate_orbits(&orbits, "COM", 0)
}


fn _calculate_orbits(orbits: &HashMap<&str, Vec<&str>>, key: &str, total: i64) -> i64 {
    let mut final_total = total;
    if let Some(orbit) = orbits.get(key) {
        for o in orbit {
            final_total += _calculate_orbits(orbits, o, total + 1);
        }
    }
    final_total
}


fn calculate_orbital_transfers(orbits: &HashMap<&str, Vec<&str>>) -> i64 {
    let you = find_orbit(orbits, "YOU") - 1;
    let santa = find_orbit(orbits, "SAN") - 1;
    let lco = find_orbit(orbits, &find_lco(orbits, "SAN", "YOU")[..]);
    you + santa - 2 * lco
}


fn find_orbit(orbits: &HashMap<&str, Vec<&str>>, target: &str) -> i64 {
    _find_orbit(orbits, "COM", target, 0).unwrap()
}


fn _find_orbit(orbits: &HashMap<&str, Vec<&str>>, root: &str, target: &str, total: i64) -> Option<i64> {
    if let Some(orbit) = orbits.get(root) {

        for o in orbit {
            if *o == target {
                return Some(total + 1)
            }

            if let Some(result) = _find_orbit(orbits, o, target, total + 1) {
                return Some(result);
            }
        }
    }
    None
}


fn find_lco(orbits: &HashMap<&str, Vec<&str>>, target1: &str, target2: &str) -> String {    
    _find_lco(orbits, "COM", target1, target2).unwrap()
}


fn _find_lco(orbits: &HashMap<&str, Vec<&str>>, root: &str, n1: &str, n2: &str) -> Option<String> {
    if n1 == root || n2 == root {
        return Some(root.to_string());
    }

    let mut results = Vec::new();

    if let Some(orbit) = orbits.get(root) {
        for o in orbit {
            if let Some(result) = _find_lco(orbits, &o, n1, n2) {
                results.push(result);
            }
        }
    }

    if results.len() > 1 {
        return Some(root.to_string());
    }

    if results.len() == 1 {
        return results.pop();
    }

    None
}