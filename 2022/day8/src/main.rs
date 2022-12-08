use std::{env, fs};

fn task_1(content: &str) -> u32 {
    let grid: Vec<_> = content.split("\n").map(|r| r.trim()).collect();
    let h = grid.len();
    let w = grid[0].len();

    let get_char = |r: usize, c: usize| grid[r].chars().nth(c).unwrap();

    let mut count_visible = 0;

    for r in 0..h {
        for c in 0..w {
            let ch = get_char(r, c);

            let visible = (0..r).all(|i| get_char(i, c) < ch)
                || (r + 1..h).all(|i| get_char(i, c) < ch)
                || (0..c).all(|i| get_char(r, i) < ch)
                || (c + 1..w).all(|i| get_char(r, i) < ch);

            if visible {
                count_visible += 1;
            }
        }
    }

    return count_visible;
}

fn task_2(content: &str) -> usize {
    let grid: Vec<_> = content.split("\n").map(|r| r.trim()).collect();
    let h = grid.len();
    let w = grid[0].len();

    let get_char = |r: usize, c: usize| grid[r].chars().nth(c).unwrap();
    let mut max_scenic_score = 0;

    for r in 0..h {
        for c in 0..w {
            let ch = get_char(r, c);

            let up = r - (0..r).rev().find(|i| get_char(*i, c) >= ch).unwrap_or(0);

            let down = (r + 1..h).find(|i| get_char(*i, c) >= ch).unwrap_or(h - 1) - r;

            let left = c - (0..c).rev().find(|i| get_char(r, *i) >= ch).unwrap_or(0);

            let right = (c + 1..w).find(|i| get_char(r, *i) >= ch).unwrap_or(w - 1) - c;

            // println!("(r={},c={}) ch={}: u={}, d={}, l={}, r={}", r, c, ch, up, down, left, right);

            let scenic_score = up * down * left * right;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    return max_scenic_score;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day6. Task1: {}", task_1(&content));
    println!("Day6. Task2: {}", task_2(&content));
}
