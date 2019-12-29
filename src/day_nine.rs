use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};


#[cfg(test)]
mod test_nine {
    use super::{run_1, Computer, run_2};

    #[test]
    fn test_sixteen_digit() {
        let mut computer = Computer { pc: 0, relative_base: 0, memory:
                                      vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0] };
        assert_eq!(
            1_219_070_632_396_864, computer.run([].to_vec()).unwrap()
        )
    }


    #[test]
    fn test_quine() {
        let mut computer = Computer { pc: 0, relative_base: 0,
                                      memory: vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99] };
        assert_eq!(
            99, computer.run([].to_vec()).unwrap()
        )
    }


    #[test]
    fn test_input() {
        let mut computer = Computer { pc: 0, relative_base: 0,
                                      memory: vec![109,988,209,12,9,1000,209,6,209,3,203,0,204,0, 99] };
        assert_eq!(
            1, computer.run([1].to_vec()).unwrap()
        )
    }
    

    #[test]
    fn test_large() {
        let mut computer = Computer { pc: 0, relative_base: 0,
                                      memory: vec![104, 1_125_899_906_842_624, 99] };
        assert_eq!(
            1_125_899_906_842_624, computer.run([].to_vec()).unwrap()
        )
    }


    #[test]
    fn test_run_1() {
        assert_eq!(
            2_594_708_277, run_1()
        )
    }


    #[test]
    fn test_run_2() {
        assert_eq!(
            87_721, run_2()
        )
    }
}


struct Computer {
    memory: Vec<i64>,
    relative_base: i64,
    pc: i64,
}


impl Computer {
    pub fn new() -> Computer {
        Computer { pc: 0, relative_base: 0, memory: Vec::new() }
    }

    fn load(&mut self, filename: &str) {
        self.read_program(&filename).unwrap();
    }

    fn read_program(&mut self, filename: &str) -> Result<(), Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            for opcode in line?.split(',') {
                self.memory.push(opcode
                    .trim()
                    .parse::<i64>()
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
            }
        }

        Ok(())
    }

    fn run(&mut self, mut inputs: Vec<i64>) -> Option<i64> {
        let mut output = None;
        inputs.reverse();
    
        loop {
            let opcode = self.read_memory(self.pc) % 100;
            let parameter = self.read_memory(self.pc) / 100;
    
            let mode_1 = parameter % 10;
            let mode_2 = (parameter/10) % 10;
            let mode_3 = (parameter/100) % 10;

            let first = self.get_parameter(self.pc + 1, mode_1);
            let second = self.get_parameter(self.pc + 2, mode_2);
            let third = self.get_parameter_index(self.pc + 3, mode_3);
    
            match opcode {
                1 => {
                    self.write_memory(third, first + second);
                    self.pc += 4;
                },
                2 => {
                    self.write_memory(third, first * second);
                    self.pc += 4;
                },
                3 => {
                    let location = self.get_parameter_index(self.pc+1, mode_1);
                    self.write_memory(location, inputs.pop().unwrap());
                    self.pc += 2;
                },
                4 => {
                    println!("Output {}", first);
                    output = Some(first);
                    self.pc += 2;
                },
                5 => {
                    self.pc = if 0 == first {
                        self.pc + 3
                    } else {
                        second
                    }
                },
                6 => {
                    self.pc = if 0 == first {
                        second
                    } else {
                        self.pc + 3
                    }
                },
                7 => {
                    if first < second {
                        self.write_memory(third, 1);
                    } else {
                        self.write_memory(third, 0);
                    }
                    self.pc += 4;
                },
                8 => {
                    if first == second {
                        self.write_memory(third, 1);
                    } else {
                        self.write_memory(third, 0);
                    }
                    self.pc += 4;
                },
                9 => {
                    self.relative_base += first;
                    self.pc += 2;
                },
                99 => break,
                _ => panic!("Unexpected opcode {}", opcode),
            }
        }
    
        output
    }

    fn get_parameter(&mut self, index: i64, mode: i64) -> i64 {
        let param = self.get_parameter_index(index, mode);
        self.read_memory(param)
    }

    fn get_parameter_index(&mut self, index: i64, mode: i64) -> i64 {        
        if mode == 1 {
            return index;
        }

        let param = self.read_memory(index);

        if mode == 0 {
            return param;
        }

        self.relative_base + param
    }

    fn read_memory(&mut self, index: i64) -> i64 {
        match self.memory.get(index as usize) {
            Some(x) => *x,
            None => 0
        }
    }

    fn write_memory(&mut self, index: i64, to_write: i64) {
        let i = index as usize;

        if i >= self.memory.len() {
            self.memory.resize(i + 1, 0);
        }

        self.memory[i] = to_write;
    }
}


pub fn run_1() -> i64 {
    let mut computer = Computer::new();
    computer.load("input/day_nine.txt");
    computer.run(vec![1]).unwrap()
}


pub fn run_2() -> i64 {
    let mut computer = Computer::new();
    computer.load("input/day_nine.txt");
    computer.run(vec![2]).unwrap()
}