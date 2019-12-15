use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};


#[cfg(test)]
mod test_five {
    use super::{run_intcode, run_1};

    #[test]
    fn test_intcode_one() {
        assert_eq!(
            vec![1002, 4, 3, 4, 99], run_intcode(0, &vec![1002, 4, 3, 4, 33])
        );
    }

    #[test]
    fn test_run_1() {
        assert_eq!(
            7_157_989, run_1().unwrap()
        );
    }
}


pub fn run_1() ->  Option<i64> {
    Some(run_program(1, "input/day_five.txt").unwrap())
}


pub fn run_program(input: i64, filename: &str) ->  Result<i64, Error> {
    let opcodes = read_opcodes(&filename).unwrap();
    let working = run_intcode(input, &opcodes);

    Ok(working[0])
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


fn run_intcode(input: i64, opcodes: &[i64]) -> Vec<i64> {
    let mut working = opcodes.to_vec();
    let mut i = 0;

    while i < working.len() {
        let opcode = working[i] % 100;
        let parameter = working[i] / 100;

        if opcode == 99 {
            break;
        }

        match opcode {
            1|2 => {
                let mode_1 = parameter % 10;
                let mode_2 = (parameter/10) % 10;

                let lhs = match mode_1 {
                    0 => working[working[i + 1] as usize],
                    1 => working[i + 1],
                    _ => panic!("Unexpected mode {}", mode_1),
                };

                let rhs = match mode_2 {
                    0 => working[working[i + 2] as usize],
                    1 => working[i + 2],
                    _ => panic!("Unexpected mode {}", mode_2)
                };

                let location = working[i + 3] as usize;

                match opcode {
                    1 => working[location] = lhs + rhs,
                    2 => working[location] = lhs * rhs,
                    _ => panic!("Unexpected opcode {}", opcode)
                }
                i += 4;
            },
            3|4 => {
                let location = working[i + 1] as usize;

                match opcode {
                    3 => working[location] = input,
                    4 => info!("Output {}", working[location]),
                    _ => panic!("Unexpected opcode {}", opcode)
                }
                i += 2;
            }
            _ => panic!("Unexpected opcode {}", opcode),
        }
    }

    working
}
