use std::{collections::VecDeque, env, fs};

enum ParserState {
    CRATES,
    MOVES,
}

struct Move {
    amount: usize,
    src: usize,
    dst: usize,
}

fn parse_move(line: &str) -> Move {
    let mut splitter = line.trim().split(' ');
    splitter.next(); // move
    let amount = splitter.next().unwrap().parse::<usize>().unwrap();
    splitter.next(); // from
    let src = splitter.next().unwrap().parse::<usize>().unwrap() - 1;
    splitter.next(); // to
    let dst = splitter.next().unwrap().parse::<usize>().unwrap() - 1;
    return Move { amount, src, dst };
}

fn parse(content: &str) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let mut crates: Vec<VecDeque<char>> = Vec::new();
    let mut moves = Vec::new();

    let mut parser_state = ParserState::CRATES;
    for line in content.split('\n') {
        if line.trim().is_empty() {
            parser_state = ParserState::MOVES;
            continue;
        }
        match parser_state {
            ParserState::CRATES => {
                for (idx, ch) in line.chars().enumerate() {
                    if 'A' <= ch && ch <= 'Z' {
                        let create_index = idx / 4;

                        while crates.len() < create_index + 1 {
                            crates.push(VecDeque::new());
                        }

                        crates[create_index].push_front(ch);
                    }
                }
            }
            ParserState::MOVES => {
                moves.push(parse_move(line));
            }
        }
    }
    return (crates, moves);
}

fn task_1(content: &str) -> String {
    let (mut crates, moves) = parse(content);

    for mv in moves {
        for _ in 0..mv.amount {
            let ch = crates[mv.src].pop_back().unwrap();
            crates[mv.dst].push_back(ch);
        }
    }

    return crates.iter().map(|c| c.back().unwrap()).collect::<String>();
}

fn task_2(content: &str) -> String {
    let (mut crates, moves) = parse(content);

    let mut interm_crate = Vec::new();
    for mv in moves {
        for _ in 0..mv.amount {
            let ch = crates[mv.src].pop_back().unwrap();
            interm_crate.push(ch);
        }
        while !interm_crate.is_empty() {
            crates[mv.dst].push_back(interm_crate.pop().unwrap());
        }
    }

    return crates.iter().map(|c| c.back().unwrap()).collect::<String>();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day5. Task1: {}", task_1(&content));
    println!("Day5. Task2: {}", task_2(&content));
}
