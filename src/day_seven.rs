use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};


use itertools::Itertools;


#[cfg(test)]
mod test_seven {
    use super::{run_intcode, run_1};

    #[test]
    fn test_thruster_signal_1() {
        let phases = &[4, 3, 2, 1, 0];
        let opcodes = &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];

        assert_eq!(
            43_210,
            phases
                  .iter()
                  .fold(0, |input, phase| run_intcode(opcodes, vec![*phase, input]).unwrap())
        )
    }


    #[test]
    fn test_thruster_signal_2() {
        let phases = &[0, 1, 2, 3, 4];
        let opcodes = &[3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0];

        assert_eq!(
            54_321,
            phases
                  .iter()
                  .fold(0, |input, phase| run_intcode(opcodes, vec![*phase, input]).unwrap())
        )
    }


    #[test]
    fn test_thruster_signal_3() {
        let phases = &[1, 0, 4, 3, 2];
        let opcodes = &[3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,1, 33, 31,
                        31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0];

        assert_eq!(
            65_210,
            phases
                  .iter()
                  .fold(0, |input, phase| run_intcode(opcodes, vec![*phase, input]).unwrap())
        )
    }

    #[test]
    fn test_run_1() {
        assert_eq!(
            46_248, run_1()
        )
    }
}


pub fn run_1() -> i64 {
    let all_phases = (0..5).permutations(5);

    all_phases
        .map(|phases| {
            phases
                .iter()
                .fold(0, |input, phase| run_program("input/day_seven.txt", vec![*phase, input]).unwrap())
        })
        .max()
        .unwrap()
}


pub fn run_program(filename: &str, inputs: Vec<i64>) ->  Option<i64> {
    let opcodes = read_opcodes(&filename).unwrap();
    run_intcode(&opcodes, inputs)
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


fn run_intcode(opcodes: &[i64], mut inputs: Vec<i64>) -> Option<i64> {
    let mut working = opcodes.to_vec();
    let mut output = None;
    let mut i = 0;
    inputs.reverse();

    loop {
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
                    3 => working[location] = inputs.pop().unwrap(),
                    4 => output = Some(working[location]),
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

    output
}


fn get_parameter(program: &[i64], index: usize, mode: i64) -> i64 {
    match mode {
        0 => program[program[index] as usize],
        1 => program[index],
        _ => panic!("Unexpected mode {}", mode),
    }
}
