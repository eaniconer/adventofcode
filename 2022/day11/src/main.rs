use log::debug;
use std::{cmp::Reverse, collections::VecDeque, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day9. Task1: {}", task_1(&content));
    println!("Day9. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> i32 {
    let mut game = Game::new(parse_monkeys(content));
    game.print();

    for i in 0..20 {
        println!("Round {}:", i + 1);
        game.play_round();
        game.print();
    }

    game.print_inspects_info();

    let mut counts = game.inspect_count;
    counts.sort_by_key(|k| Reverse(*k));

    return counts[0] * counts[1];
}

fn task_2(content: &str) -> i64 {
    let mut game = Game::new(parse_monkeys(content));

    for i in 0..10000 {
        game.play_round_2();

        if (i + 1) % 100 == 0 {
            println!("== Round: {}", i + 1);
        }
        if (i + 1) % 1000 == 0 {
            println!("== After round {} ==", i + 1);
            game.print_inspects_info();
        }
    }

    let mut counts = game.inspect_count;
    counts.sort_by_key(|k| Reverse(*k));

    return (counts[0] as i64) * (counts[1] as i64);
}

// Task 1
struct Game {
    monkeys: Vec<Monkey>,
    inspect_count: Vec<i32>,
}

impl Game {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let sz = monkeys.len();
        Self {
            monkeys,
            inspect_count: vec![0; sz],
        }
    }

    fn play_round(&mut self) {
        for i in 0..self.monkeys.len() {
            self.inspect_count[i] += self.monkeys[i].items.len() as i32;
            debug!("Monkey {}:", i);
            while let Some(item) = self.monkeys[i].items.pop_front() {
                let monkey = &self.monkeys[i];
                debug!("  Monkey inspects item with a worry level of {}", item);
                let item = monkey.op.eval(item);
                debug!("  Worry level has been changed to {}", item);
                let item = item / 3;
                debug!(
                    "  Monkey gets bored with item. Worry level is reduced to {}",
                    item
                );
                let nxt_monkey = monkey.test.select(item) as usize;
                debug!(
                    "  Item with worry level {} is thrown to monkey {}",
                    item, nxt_monkey
                );
                self.monkeys[nxt_monkey].items.push_back(item);
            }
        }
    }

    fn play_round_2(&mut self) {
        let safe_divisor: i64 = self.monkeys.iter().map(|m| m.test.divisor).product();

        for i in 0..self.monkeys.len() {
            self.inspect_count[i] += self.monkeys[i].items.len() as i32;
            debug!("Monkey {}:", i);
            while let Some(item) = self.monkeys[i].items.pop_front() {
                let monkey = &self.monkeys[i];
                debug!("  Monkey inspects item with a worry level of {}", item);
                let item = monkey.op.eval(item);
                debug!("  Worry level has been changed to {}", item);
                let item = item % safe_divisor;
                debug!(
                    "  Monkey gets bored with item. Worry level is reduced to {}",
                    item
                );
                let nxt_monkey = monkey.test.select(item) as usize;
                debug!(
                    "  Item with worry level {} is thrown to monkey {}",
                    item, nxt_monkey
                );
                self.monkeys[nxt_monkey].items.push_back(item);
            }
        }
    }

    fn print(&self) {
        for (idx, monkey) in self.monkeys.iter().enumerate() {
            print!("Monkey {}:", idx);
            for item in &monkey.items {
                print!(" {}", *item);
            }
            println!("");
        }
        println!("")
    }

    fn print_inspects_info(&self) {
        for (i, cnt) in self.inspect_count.iter().enumerate() {
            println!("Monkey {} inspected items {} time(s)", i, cnt);
        }
    }
}

// Common

struct Monkey {
    items: VecDeque<i64>,
    op: Op,
    test: Test,
}

impl Monkey {
    fn new(items: VecDeque<i64>, op: Op, test: Test) -> Self {
        Self { items, op, test }
    }
}

enum Operand {
    Old,
    Number(i64),
}

impl Operand {
    fn eval(&self, old: i64) -> i64 {
        match self {
            Operand::Old => old,
            Operand::Number(x) => *x,
        }
    }
}

enum Op {
    Add(Operand, Operand),
    Mult(Operand, Operand),
}

impl Op {
    fn eval(&self, old: i64) -> i64 {
        match self {
            Op::Add(lhs, rhs) => lhs.eval(old) + rhs.eval(old),
            Op::Mult(lhs, rhs) => lhs.eval(old) * rhs.eval(old),
        }
    }
}

struct Test {
    divisor: i64,
    true_monkey: i32,
    false_monkey: i32,
}

impl Test {
    fn select(&self, x: i64) -> i32 {
        if x % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

fn parse_monkeys(content: &str) -> Vec<Monkey> {
    let mut splitter = content.split("\n");
    let mut monkeys = Vec::new();

    loop {
        splitter.next(); // skip 'Monkey N:'

        let items = parse_items(splitter.next().unwrap());
        let op = parse_op(splitter.next().unwrap());
        let test = parse_test(
            splitter.next().unwrap(),
            splitter.next().unwrap(),
            splitter.next().unwrap(),
        );

        monkeys.push(Monkey::new(items, op, test));

        if splitter.next().is_none() {
            break;
        }
    }

    monkeys
}

fn parse_items(line: &str) -> VecDeque<i64> {
    assert!(line.starts_with("  Starting items: "));
    let mut splitter = line.split(": ");
    splitter.next(); // skip 'Starting items: '

    let raw_items = splitter.next().unwrap();
    raw_items
        .split(',')
        .map(|item| item.trim().parse::<i64>().unwrap())
        .collect()
}

fn parse_op(line: &str) -> Op {
    let expr = line.split(" = ").nth(1).unwrap();
    let mut splitter = expr.trim().split(' ');

    let lhs = parse_operand(splitter.next().unwrap());
    let op = splitter.next().unwrap();
    let rhs = parse_operand(splitter.next().unwrap());

    match op {
        "*" => Op::Mult(lhs, rhs),
        "+" => Op::Add(lhs, rhs),
        _ => unreachable!(),
    }
}

fn parse_operand(line: &str) -> Operand {
    if line == "old" {
        Operand::Old
    } else {
        Operand::Number(line.parse().unwrap())
    }
}

fn parse_test(cond: &str, true_monkey: &str, false_monkey: &str) -> Test {
    let divisor = parse_last_word_as_number(cond) as i64;
    let true_monkey = parse_last_word_as_number(true_monkey);
    let false_monkey = parse_last_word_as_number(false_monkey);
    Test {
        divisor,
        true_monkey,
        false_monkey,
    }
}

fn parse_last_word_as_number(line: &str) -> i32 {
    line.split(' ').rev().next().unwrap().parse().unwrap()
}
