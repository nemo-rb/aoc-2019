use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};


#[cfg(test)]
mod intcode_tests {
    use super::{run_intcode, run_1, run_2};


    #[test]
    fn test_incode_one() {
        assert_eq!(
            vec![2, 0, 0, 0, 99], run_intcode(&vec![1, 0, 0, 0, 99])
        );
    }


    #[test]
    fn test_incode_two() {
        assert_eq!(
            vec![2, 3, 0, 6, 99], run_intcode(&vec![2, 3, 0, 3, 99])
        );
    }


    #[test]
    fn test_incode_three() {
        assert_eq!(
            vec![2, 4, 4, 5, 99, 9801], run_intcode(&vec![2, 4, 4, 5, 99, 0])
        );
    }


    #[test]
    fn test_incode_four() {
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99], run_intcode(&vec![1,1,1,4,99,5,6,0,99])
        );
    }

    #[test]
    fn test_run_program_1() {
        assert_eq!(
            6627023, run_1().unwrap()
        );
    }

    #[test]
    fn test_run_program_2() {
        assert_eq!(
            19690720, run_2().unwrap()
        );
    }
}


pub fn run_1() ->  Option<i64> {
    Some(run_program(12, 2, "input/day_two.txt").unwrap())
}


pub fn run_2() ->  Option<i64> {
    let mut result = None;

    for i in 0..99 {
        for j in 0..99 {
            if run_program(i, j, "input/day_two.txt").unwrap() == 19690720 {
                 result = Some(19690720);
                 break;
            }
        }
    }

    result
}


pub fn run_program(noun: i64, verb: i64, filename: &str) ->  Result<i64, Error> {
    let opcodes = init_intcode(noun, verb, filename).unwrap();
    let working = run_intcode(&opcodes);

    Ok(working[0])
}


pub fn init_intcode(noun: i64, verb: i64, filename: &str) ->  Result<Vec<i64>, Error> {
    let mut opcodes = read_opcodes(&filename).unwrap();
    opcodes[1] = noun;
    opcodes[2] = verb;

    Ok(opcodes)
}


fn read_opcodes(filename: &str) -> Result<Vec<i64>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut opcodes = Vec::new();

    for line in reader.lines() {
        for opcode in line?.split(',') {
            opcodes.push(opcode
                .trim()
                .parse::<i64>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
        }
    }

    Ok(opcodes)
}


fn run_intcode(opcodes: &Vec<i64>) -> Vec<i64> {

    let mut working = opcodes.to_vec();

    for i in (0..working.len()).step_by(4) {
        let opcode = working[i];

        if opcode == 99 {
            break;
        }

        match opcode {
            1|2 => {
                let lhs = working[working[i + 1] as usize];
                let rhs = working[working[i + 2] as usize];
                let location = working[i + 3] as usize;

                match opcode {
                    1 => working[location] = lhs + rhs,
                    2 => working[location] = lhs * rhs,
                    _ => panic!("Unexpected opcode {}", opcode),
                }
            }
            _ => panic!("Unexpected opcode {}", opcode),
        }
    }

    working
}
