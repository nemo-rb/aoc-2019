use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, stdin};


#[cfg(test)]
mod test_five {
    use super::{run_intcode, run_1};

    #[test] #[ignore]
    fn test_intcode_one() {
        assert_eq!(
            vec![1002, 4, 3, 4, 99], run_intcode(&vec![1002, 4, 3, 4, 33])
        );
    }

    #[test] #[ignore]
    fn test_run_1() {
        assert_eq!(
            7_157_989, run_1().unwrap()
        );
    }

    #[test] #[ignore]
    fn test_eq_eight() {
        run_intcode(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

        assert!(
            true
        );
    }

    #[test] #[ignore]
    fn test_lt_eight() {
        run_intcode(&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

        assert!(
            true
        );
    }

    #[test] #[ignore]
    fn test_eq_eight_immediate() {
        run_intcode(&vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);

        assert!(
            true
        );
    }

    #[test] #[ignore]
    fn test_lt_eight_immediate() {
        run_intcode(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);

        assert!(
            true
        );
    }

    #[test] #[ignore]
    fn test_jump_test() {
        run_intcode(&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);

        assert!(
            true
        );
    }

    #[test] #[ignore]
    fn test_jump_test_immediate() {
        run_intcode(&vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);

        assert!(
            true
        );
    }

    #[test] #[ignore]
    fn test_long_program() {
        run_intcode(&vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]);

        assert!(
            true
        );
    }
}


pub fn run_1() ->  Option<i64> {
    Some(run_program("input/day_five.txt").unwrap())
}


pub fn run_2() ->  Option<i64> {
    Some(run_program("input/day_five.txt").unwrap())
}


fn run_program(filename: &str) ->  Result<i64, Error> {
    let opcodes = read_opcodes(&filename).unwrap();
    let working = run_intcode(&opcodes);

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


fn run_intcode(opcodes: &[i64]) -> Vec<i64> {
    let mut working = opcodes.to_vec();
    let mut i = 0;

    while i < working.len() {
        let opcode = working[i] % 100;
        let parameter = working[i] / 100;

        let mode_1 = parameter % 10;
        let mode_2 = (parameter/10) % 10;

        match opcode {
            1|2|7|8 => {
                let first = get_parameter(&working, i+1, mode_1);
                let second = get_parameter(&working, i+2, mode_2);
                let location = working[i + 3] as usize;

                match opcode {
                    1 => working[location] = first + second,
                    2 => working[location] = first * second,
                    7 => {
                        if first < second {
                            working[location] = 1;
                        } else {
                            working[location] = 0;
                        }
                    },
                    8 => {
                        if first == second {
                            working[location] = 1;
                        } else {
                            working[location] = 0;
                        }
                    },
                    _ => panic!("Unexpected opcode {}", opcode)
                }
                i += 4;
            },
            3|4 => {
                let location = working[i + 1] as usize;

                match opcode {
                    3 => working[location] = read_input(),
                    4 => println!("Output {}", working[location]),
                    _ => panic!("Unexpected opcode {}", opcode)
                }

                i += 2;
            },
            5|6 => {
                let condition = get_parameter(&working, i+1, mode_1);
                let location = get_parameter(&working, i+2, mode_2) as usize;

                match opcode {
                    5 => {
                        if 0 != condition {
                            i = location;
                        } else {
                            i += 3;
                        }
                    },
                    6 => {
                        if 0 == condition {
                            i = location;
                        } else {
                            i += 3;
                        }
                    },
                    _ => panic!("Unexpected opcode")
                }
            },
            99 => break,
            _ => panic!("Unexpected opcode {}", opcode),
        }
    }

    working
}


fn read_input() -> i64 {
    println!("Please enter your input:");

    let mut n = String::new();
    stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    n.trim().parse().expect("invalid input")
}


fn get_parameter(program: &[i64], index: usize, mode: i64) -> i64 {
    match mode {
        0 => program[program[index] as usize],
        1 => program[index],
        _ => panic!("Unexpected mode {}", mode),
    }
}
