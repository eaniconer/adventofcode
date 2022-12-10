use std::{
    env,
    fmt::{Debug, Display, Write},
    fs,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day9. Task1: {}", task_1(&content));
    println!("Day9. Task2: {}", task_2(&content));
}

enum Op {
    Addx(i32, i32),
    Noop(i32),
}

impl Op {
    fn from(s: &str) -> Op {
        let mut splitter = s.trim().split(' ');
        match splitter.next().unwrap() {
            "noop" => Op::Noop(1),
            "addx" => Op::Addx(splitter.next().unwrap().parse::<i32>().unwrap(), 2),
            cmd => panic!("Unexpected command: '{}'", cmd),
        }
    }
}

// Task 1

fn task_1(content: &str) -> i32 {
    let mut cpu = Cpu::new();

    let mut sig_strength = 0;
    for line in content.split('\n') {
        let prev = (cpu.cycles_done + 20) / 40;
        let prev_reg_x = cpu.reg_x;
        cpu.dispatcher(Op::from(line));
        let next = (cpu.cycles_done + 20) / 40;
        if next != prev {
            sig_strength += prev_reg_x * (prev * 40 + 20)
        }
    }

    sig_strength
}

struct Cpu {
    reg_x: i32,
    cycles_done: i32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            reg_x: 1,
            cycles_done: 0,
        }
    }

    fn dispatcher(&mut self, op: Op) {
        match op {
            Op::Addx(v, c) => self.addx(v, c),
            Op::Noop(c) => self.noop(c),
        }
    }

    fn addx(&mut self, value: i32, cycles: i32) {
        self.make_cycles(cycles);
        self.reg_x += value;
    }

    fn noop(&mut self, cycles: i32) {
        self.make_cycles(cycles)
    }

    fn make_cycles(&mut self, cycles: i32) {
        self.cycles_done += cycles;
    }
}

// Task 2

fn task_2(program: &str) -> u32 {
    let mut renderer = Renderer::new(CrtScreen::new(6, 40));
    let mut cpu = Cpu2::new();
    cpu.load(Program::from(program));

    for _ in 0..240 {
        renderer.do_cycle(cpu.reg.x);
        cpu.do_cycle()
    }

    println!("{}", renderer.screen);

    0
}

struct CrtScreen {
    height: usize,
    width: usize,
    buffer: Vec<char>,
}

impl CrtScreen {
    fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            buffer: vec!['.'; height * width],
        }
    }
}

struct Renderer {
    screen: CrtScreen,
    cur_pos: usize,
}

impl Renderer {
    fn new(screen: CrtScreen) -> Self {
        Self { screen, cur_pos: 0 }
    }

    fn do_cycle(&mut self, sprite_pos: i32) {
        let p = (self.cur_pos % 40) as i32;
        if sprite_pos - 1 <= p && p <= sprite_pos + 1 {
            self.screen.buffer[self.cur_pos] = '#';
        }
        self.cur_pos += 1;
    }
}

impl Display for CrtScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                f.write_str(&self.buffer[i * self.width + j].to_string());
            }
            f.write_char('\n');
        }
        Result::Ok(())
    }
}

struct Registers {
    x: i32,
    ip: usize, // instruction pointer (points to next instruction to execute)
    icc: i32,  // instuction cycle counter
}

impl Registers {
    fn new() -> Self {
        Self {
            x: 1,
            ip: 0,
            icc: 0,
        }
    }
}

struct Cpu2 {
    reg: Registers,
    program: Option<Program>,
}

impl Cpu2 {
    fn new() -> Self {
        Self {
            reg: Registers::new(),
            program: None,
        }
    }

    fn load(&mut self, program: Program) {
        self.program = Some(program);
    }

    fn do_cycle(&mut self) {
        if let Some(program) = &self.program {
            if let Some(op) = program.ops.get(self.reg.ip) {
                match op {
                    Op::Addx(v, c) => {
                        self.reg.icc += 1;
                        if self.reg.icc == *c {
                            self.reg.icc = 0;
                            self.reg.ip += 1;
                            self.reg.x += v;
                        }
                    }
                    Op::Noop(_) => {
                        self.reg.icc = 0;
                        self.reg.ip += 1;
                    }
                }
            }
        }
    }
}

struct Program {
    ops: Vec<Op>,
}

impl Program {
    fn from(text: &str) -> Self {
        Self {
            ops: text.split('\n').map(|line| Op::from(line)).collect(),
        }
    }
}
