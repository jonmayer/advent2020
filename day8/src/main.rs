use advent;

#[derive(Clone, Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone, Debug)]
struct Instruction {
    operation: Operation,
    operand: i64,
}

impl Instruction {
    fn new() -> Instruction {
        Instruction { operation: Operation::Nop, operand: 0 }
    }

    fn parse(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        self.operation = match parts[0] {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => panic!("Unrecognized operation: {}", parts[0]),
        };
        self.operand = parts[1].parse::<i64>().unwrap();
    }
}

fn line_to_instruction(line: &str) -> Instruction {
    let mut i = Instruction::new();
    i.parse(line);
    return i;
}

#[derive(Clone)]
struct Computer{
    program: Vec<Instruction>,
    visited: Vec<bool>,
    acc: i64,
    pc: i64,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            program: Vec::new(),
            visited: Vec::new(),
            acc: 0,
            pc: 0,
        }
    }

    fn load_program(&mut self, text: &str) {
        self.program = text.lines()
            .map(|line| line_to_instruction(line))
            .collect();
        self.visited = self.program.iter().map(|_| false).collect();
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.visited = self.program.iter().map(|_| false).collect();
    }

    // Runs a single instruction step.
    // Returns true on the halt condition (ie. if the next instruction
    // to be executed has already been visited).
    fn step(&mut self) -> bool {
        let i = self.program.get(self.pc as usize).unwrap();
        match i.operation {
            Operation::Acc => self.acc += i.operand,
            Operation::Jmp => self.pc += i.operand - 1,
            Operation::Nop => (),
            _ => panic!("Bad instruction {:?}", i),
        };
        self.visited[self.pc as usize] = true;
        self.pc += 1;
        let r = self.visited.get(self.pc as usize);
        return match r {
            Some(v) => *v,
            None => false,
        };
    }

    fn run(&mut self) -> bool {
        while !self.step() {
            if self.pc as usize == self.program.len() {
                println!("Completed!");
                return true;
            }
        }
        return false;
    }
}



fn main() {
    let content = advent::load_input();
    let mut computer = Computer::new();
    computer.load_program(&content);
    computer.run();
    let part1_result = computer.acc;
    dbg!(part1_result);

    // Try all mutations of the program to see if any complete.
    computer.reset();
    let mut part2_result: i64 = 0;
    for i in 0..computer.program.len() {
        let mut mutant = computer.clone();
        mutant.program[i as usize].operation = match computer.program[i as usize].operation {
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
            Operation::Acc => Operation::Acc,
        };
        if mutant.run() {
            part2_result = mutant.acc;
            break;
        }
    }
    dbg!(part2_result);
}
