use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use itertools::Itertools;


#[cfg(test)]
mod tests {
    use super::{run_1};

    #[test]
    fn test_run_1() {
        assert_eq!(
            run_1(), Some(1_215)
        );
    }

}


pub fn run_1() -> Option<usize> {
    let filename = "input/day_eight.txt";
    let image = read_image(filename).unwrap();
    Some(verify_image(&image))
}


pub fn run_2() -> Option<usize> {
    let filename = "input/day_eight.txt";
    let image = read_image(filename).unwrap();
    let decoded = decode_image(&image);

    for (i, p) in decoded.iter().enumerate() {
        if *p == 0 {
            print!("  ");
            continue;
        }
        if i != 0 && i % 25 == 0 {
            println!();
        }
        print!("{} ", p);
    }
    println!();

    Some(0)
}


fn read_image(filename: &str) -> Result<Vec<Vec<u32>>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut layers = Vec::new();

    for line in reader.lines() {
        for layer in line?.chars().chunks(25*6).into_iter() {
            layers.push(
                layer
                .map(|c|
                    c
                    .to_digit(10)
                    .unwrap()
                )
                .collect()
            );
        }
    }

    Ok(layers)
}


fn verify_image(layers: &[Vec<u32>]) -> usize {
    let minimum = layers
                    .iter()
                    .min_by_key(|layer|
                                 count_values(layer, 0)
                                ).unwrap();

    
    count_values(&minimum, 1) * count_values(&minimum, 2)
}


fn count_values(input: &[u32], to_count: u32) -> usize {
    input
    .iter()
    .filter(|&c| *c == to_count)
    .count()
}


fn decode_image(layers: &[Vec<u32>]) -> [u32; 150] {
    let mut decoded: [u32; 25 * 6] = [2; 25 * 6];

    for layer in layers {
        for (i, p) in layer.iter().enumerate() {
            if decoded[i] != 2 {
                continue;
            }

            decoded[i] = cmp::min(*p, decoded[i]);
        }
    }

    decoded
}