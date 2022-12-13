use std::{cmp::Ordering, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day13. Task1: {}", task_1(&content));
    println!("Day13. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> u32 {
    let mut splitter = content.split("\n");

    let mut answer = 0;
    let mut iter = 0;
    loop {
        iter += 1;
        let lhs = Value::from(splitter.next().unwrap().trim());
        let rhs = Value::from(splitter.next().unwrap().trim());
        if lhs < rhs {
            answer += iter;
        }

        if splitter.next().is_none() {
            break;
        }
    }

    answer
}

fn task_2(content: &str) -> usize {
    let mut splitter = content.split("\n");

    let mut values = Vec::new();
    loop {
        let lhs = Value::from(splitter.next().unwrap().trim());
        let rhs = Value::from(splitter.next().unwrap().trim());

        values.push(lhs);
        values.push(rhs);

        if splitter.next().is_none() {
            break;
        }
    }

    values.push(Value::from("[[2]]"));
    values.push(Value::from("[[6]]"));

    values.sort();

    let mut answer = 1;
    for (idx, val) in values.iter().enumerate() {
        if *val == Value::from("[[2]]") {
            answer *= idx + 1;
        }
        if *val == Value::from("[[6]]") {
            answer *= idx + 1;
        }
    }

    answer
}

#[derive(Debug)]
enum Value {
    Int(i32),
    Lst(Vec<Value>),
}

impl Value {
    fn from(line: &str) -> Self {
        Value::_parse_list(line).unwrap().0
    }

    fn _parse_list(line: &str) -> Option<(Value, &str)> {
        // println!("Try parsing: {line}");
        if !line.starts_with("[") {
            panic!("Open bracket expected");
        }
        let mut items = Vec::new();
        let mut line = line.get(1..)?;
        loop {
            if line.starts_with("]") {
                return Some((Value::Lst(items), line.get(1..)?));
            }

            if line.starts_with(",") {
                line = line.get(1..)?
            }

            line = line.trim();

            let (v, rest) = if line.starts_with("[") {
                Value::_parse_list(line)?
            } else {
                Value::_parse_value(line)?
            };
            items.push(v);
            line = rest;
        }
    }

    fn _parse_value(line: &str) -> Option<(Value, &str)> {
        let i = line.find(|c: char| !c.is_digit(10)).unwrap_or(line.len());
        let v = Value::Int(line.get(..i)?.parse().unwrap());
        Some((v, line.get(i..)?))
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Value::Int(x) => match other {
                Value::Int(y) => x.partial_cmp(y),
                _ => Value::Lst(vec![Value::Int(*x)]).partial_cmp(other),
            },
            Value::Lst(lv) => match other {
                Value::Int(x) => self.partial_cmp(&Value::Lst(vec![Value::Int(*x)])),
                Value::Lst(rv) => {
                    let len = std::cmp::min(lv.len(), rv.len());
                    for i in 0..len {
                        let cmp = lv[i].partial_cmp(&rv[i]);
                        if let Some(Ordering::Equal) = cmp {
                            continue;
                        }
                        return cmp;
                    }
                    lv.len().partial_cmp(&rv.len())
                }
            },
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if let Some(Ordering::Equal) = self.partial_cmp(other) {
            true
        } else {
            false
        }
    }
}
