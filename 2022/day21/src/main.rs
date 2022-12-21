use std::{collections::HashMap, env, fs, ops::Add};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    // println!("Day15. Task1: {}", task_1(&content));
    println!("Day15. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> i64 {
    let mut name2expr: HashMap<String, Expr> = HashMap::from_iter(
        content
            .split("\n")
            .map(|line| line.split_once(": ").unwrap())
            .map(|(name, raw_expr)| (name.to_string(), Expr::parse(raw_expr))),
    );

    let mut stack = Vec::new();
    stack.push("root".to_owned());

    while !stack.is_empty() {
        let last = stack.last().unwrap();

        let relax = match name2expr.get(last).unwrap() {
            Expr::Number(x) => {
                stack.pop();
                continue;
            }
            Expr::Math(lhs, op, rhs) => {
                let lhs = match name2expr.get(lhs).unwrap() {
                    Expr::Number(x) => *x,
                    Expr::Math(_, _, _) => {
                        stack.push(lhs.to_string());
                        continue;
                    }
                };
                let rhs = match name2expr.get(rhs).unwrap() {
                    Expr::Number(x) => *x,
                    Expr::Math(_, _, _) => {
                        stack.push(rhs.to_string());
                        continue;
                    }
                };
                op.eval(lhs, rhs)
            }
        };
        *name2expr.get_mut(last).unwrap() = Expr::Number(relax);
    }

    match name2expr.get(&"root".to_owned()).unwrap() {
        Expr::Number(x) => *x,
        Expr::Math(_, _, _) => panic!("Root is not evaluated"),
    }
}

fn check_guess(name2expr: &HashMap<String, Expr>, my_guess: i64) -> (i64, i64) {
    let mut name2expr = name2expr.clone();
    *name2expr.get_mut("humn").unwrap() = Expr::Number(my_guess);

    let mut stack = Vec::new();
    stack.push("root".to_owned());

    while !stack.is_empty() {
        let last = stack.last().unwrap();

        let relax = match name2expr.get(last).unwrap() {
            Expr::Number(x) => {
                stack.pop();
                continue;
            }
            Expr::Math(lhs, op, rhs) => {
                let lhs = match name2expr.get(lhs).unwrap() {
                    Expr::Number(x) => *x,
                    Expr::Math(_, _, _) => {
                        stack.push(lhs.to_string());
                        continue;
                    }
                };
                let rhs = match name2expr.get(rhs).unwrap() {
                    Expr::Number(x) => *x,
                    Expr::Math(_, _, _) => {
                        stack.push(rhs.to_string());
                        continue;
                    }
                };

                if last == "root" {
                    return (lhs, rhs);
                }
                op.eval(lhs, rhs)
            }
        };

        *name2expr.get_mut(last).unwrap() = Expr::Number(relax);
    }
    unreachable!();
}

fn task_2(content: &str) -> i64 {
    let mut name2expr: HashMap<String, Expr> = HashMap::from_iter(
        content
            .split("\n")
            .map(|line| line.split_once(": ").unwrap())
            .map(|(name, raw_expr)| (name.to_string(), Expr::parse(raw_expr))),
    );

    println!("MY ANSWER: {:?}", check_guess(&name2expr, 3617613952379));
    println!("MY ANSWER: {:?}", check_guess(&name2expr, 3617613952378)); // both answers, but only this acceptable

    let mut min_guess = -100000000;
    let (lc1, rc1) = check_guess(&name2expr, min_guess);
    if lc1 == rc1 {
        return min_guess;
    }

    let mut max_guess = 100000000;
    let (lc2, rc2) = check_guess(&name2expr, max_guess);
    if lc2 == rc2 {
        return max_guess;
    }

    let guess_affects_root_lhs = lc1 != lc2;
    let guess_affects_root_rhs = rc1 != rc2;

    if guess_affects_root_lhs && guess_affects_root_rhs {
        panic!("not supported");
    }
    if !guess_affects_root_lhs && !guess_affects_root_lhs {
        panic!("guesses don't affect subtrees (or we are just unlucky with the guesses)");
    }

    let target = if guess_affects_root_lhs {
        rc1
    } else if guess_affects_root_rhs {
        lc1
    } else {
        unreachable!()
    };

    let affected_number = |guess: i64| {
        let ans = check_guess(&name2expr, guess);
        if guess_affects_root_lhs {
            ans.0
        } else {
            ans.1
        }
    };

    let mut affected_1 = if guess_affects_root_lhs { lc1 } else { rc1 };

    let mut affected_2 = if guess_affects_root_lhs { lc2 } else { rc2 };

    while (affected_1 < target && affected_2 < target)
        || (affected_1 > target && affected_2 > target)
    {
        min_guess *= 10; // < 0
        max_guess *= 10; // > 0

        affected_1 = affected_number(min_guess);
        affected_2 = affected_number(max_guess);

        if affected_1 == target {
            return min_guess;
        }
        if affected_2 == target {
            return max_guess;
        }
    }

    println!("target subtree {}", target);

    // bin search

    while min_guess < max_guess {
        println!("Guess between {} and {}", min_guess, max_guess);
        println!("a1 ~ t ~ a2: {} ~ {} ~ {}", affected_1, target, affected_2);

        let guess = (min_guess + max_guess) / 2;
        let affected = affected_number(guess);

        if affected == target {
            return guess;
        }

        if affected < target {
            if affected_1 < target {
                assert!(affected_2 > target);
                min_guess = guess;
                affected_1 = affected;
                continue;
            }

            if affected_2 < target {
                assert!(affected_1 > target);
                max_guess = guess;
                affected_2 = affected;
                continue;
            }
        } else {
            if affected_1 > target {
                assert!(affected_2 < target);
                min_guess = guess;
                affected_1 = affected;
                continue;
            }

            if affected_2 > target {
                assert!(affected_1 < target);
                max_guess = guess;
                affected_2 = affected;
                continue;
            }
        }
    }

    todo!()
}

#[derive(Clone)]
enum Op {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Op {
    fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Plus => lhs + rhs,
            Op::Minus => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }

    fn parse(str: &str) -> Self {
        match str.trim() {
            "+" => Op::Plus,
            "-" => Op::Minus,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!("Unexpected string '{}'", str),
        }
    }
}

#[derive(Clone)]
enum Expr {
    Number(i64),
    Math(String, Op, String),
}

impl Expr {
    fn parse(line: &str) -> Self {
        let line = line.trim();

        if let Ok(n) = line.parse::<i64>() {
            return Expr::Number(n);
        }

        let mut splitter = line.split(" ");

        let lhs_name = splitter.next().unwrap().to_string();
        assert!(lhs_name.len() > 0);

        let op = Op::parse(splitter.next().unwrap());

        let rhs_name = splitter.next().unwrap().to_string();
        assert!(rhs_name.len() > 0);

        Expr::Math(lhs_name, op, rhs_name)
    }
}
