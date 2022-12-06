use itertools::Itertools;
use std::{env, fs};

fn solve(content: &str, marker_len: usize) -> usize {
    let content = content.trim();

    if content.len() < marker_len {
        panic!("Invalid length of stream: {}", content);
    }

    for i in 0..content.len() {
        if let Some(window) = content.get(i..i + marker_len) {
            if window.chars().unique().count() == marker_len {
                return i + marker_len;
            }
        }
    }

    unreachable!()
}

fn task_1(content: &str) -> usize {
    const MARKER_LEN: usize = 4;
    return solve(content, MARKER_LEN);
}

fn task_2(content: &str) -> usize {
    const MARKER_LEN: usize = 14;
    return solve(content, MARKER_LEN);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day6. Task1: {}", task_1(&content));
    println!("Day6. Task2: {}", task_2(&content));
}
