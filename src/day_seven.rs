use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::thread;
use std::sync::mpsc;


use itertools::Itertools;


#[cfg(test)]
mod test_seven {
    use super::{run_intcode, run_1, run_program_2, run_2};

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
    fn test_thruster_signal_feedback() {
        let phases = &[9, 8, 7, 6, 5];
        let opcodes = vec![3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
                        27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5];

        assert_eq!(
            139_629_729,
            run_program_2(&opcodes, phases)
        )
    }


    #[test]
    fn test_thruster_signal_feedback_1() {
        let phases = &[9, 7, 8, 5, 6];
        let opcodes = vec![3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
                        -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
                        53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10];

        assert_eq!(
            18_216,
            run_program_2(&opcodes, phases)
        )
    }


    #[test]
    fn test_run_1() {
        assert_eq!(
            46_248, run_1()
        )
    }


    #[test]
    fn test_run_2() {
        assert_eq!(
            54_163_586, run_2()
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


pub fn run_2() -> i64 {
    let all_phases = (5..10).permutations(5);
    let opcodes = read_opcodes("input/day_seven.txt").unwrap();

    all_phases
        .map(|phases| {
            run_program_2(&opcodes, &phases)
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
                        working[location] = if first < second {
                            1
                        } else {
                            0
                        }
                    },
                    8 => {
                        working[location] = if first == second {
                            1
                        } else {
                            0
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
                        i = if 0 == condition {
                            i + 3
                        } else {
                            location
                        }
                    },
                    6 => {
                        i = if 0 == condition {
                            location
                        } else {
                            i + 3
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


fn run_program_2(opcodes: &[i64], phases: &[i64]) -> i64 {
    let mut join_handle = None;

    let (base_tx, base_rx) = mpsc::sync_channel(2);
    let mut old_tx = base_tx.clone();
    let mut old_rx = base_rx;

    for (i, phase) in phases.iter().enumerate() {
        old_tx.send(*phase).unwrap();

        if i == 0 {
            old_tx.send(0).unwrap();
        }

        let owned_opcodes = opcodes.to_owned();
        let (new_tx, new_rx) = mpsc::sync_channel(2);

        let new_tx_n = if i == phases.len() - 1 {
            base_tx.clone()
        } else {
            new_tx.clone()
        };

        let handle = thread::spawn(move || {
            run_intcode_2(&owned_opcodes, &new_tx_n, &old_rx)
        });

        old_rx = new_rx;
        old_tx = new_tx;
        join_handle = Some(handle);
    }

    join_handle.unwrap().join().unwrap().unwrap()
}


fn run_intcode_2(opcodes: &[i64], tx: &mpsc::SyncSender<i64>, rx: &mpsc::Receiver<i64>) -> Option<i64> {
    let mut working = opcodes.to_vec();
    let mut i = 0;
    let mut output = None;

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
                        working[location] = if first < second {
                            1
                        } else {
                            0
                        }
                    },
                    _ => {
                        working[location] = if first == second {
                            1
                        } else {
                            0
                        }
                    },
                }
                i += 4;
            },
            3|4 => {
                let location = working[i + 1] as usize;

                match opcode {
                    3 => working[location] = rx.recv().unwrap(),
                    _ => {
                        let _ = tx.try_send(working[location]);
                        output = Some(working[location]);
                    },
                }

                i += 2;
            },
            5|6 => {
                let condition = get_parameter(&working, i+1, mode_1);
                let location = get_parameter(&working, i+2, mode_2) as usize;

                match opcode {
                    5 => {
                        i = if 0 == condition {
                            i + 3
                        } else {
                            location
                        }
                    },
                    _ => {
                        i = if 0 == condition {
                            location
                        } else {
                            i + 3
                        }
                    },
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
