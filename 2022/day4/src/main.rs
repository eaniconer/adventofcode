use std::{env, fs};

#[derive(Clone, Copy)]
struct Assignment {
    lhs_id: i32,
    rhs_id: i32,
}

impl Assignment {
    fn contains(&self, other: Assignment) -> bool {
        return self.lhs_id <= other.lhs_id && other.rhs_id <= self.rhs_id;
    }
}

fn parse_assignment(assignment: &str) -> Assignment {
    let mut splitter = assignment.trim().split('-');
    let lhs_id = splitter.next().unwrap().parse::<i32>().unwrap();
    let rhs_id = splitter.next().unwrap().parse::<i32>().unwrap();
    return Assignment { lhs_id, rhs_id }
}

fn parse_assignments(assignment_pair: &str) -> (Assignment, Assignment) {
    let mut splitter = assignment_pair.trim().split(',');
    let a1 = parse_assignment(splitter.next().unwrap());
    let a2 = parse_assignment(splitter.next().unwrap());
    return (a1, a2);
}

fn task_1(content: &str) -> usize {
    return content.split('\n')
        .map(parse_assignments)
        .filter(|(a1, a2)| a1.contains(*a2) || a2.contains(*a1))
        .count();
}

fn overlapped(a1: Assignment, a2: Assignment) -> bool {
    return !(a1.rhs_id < a2.lhs_id || a2.rhs_id < a1.lhs_id)
}

fn task_2(content: &str) -> usize {
    return content.split('\n')
        .map(parse_assignments)
        .filter(|(a1, a2)| overlapped(*a1, *a2))
        .count();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .expect("File not found");

    println!("Day4. Task1: {}", task_1(&content));
    println!("Day4. Task2: {}", task_2(&content));
}
