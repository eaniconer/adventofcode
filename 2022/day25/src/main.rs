use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day15. Task1: {}", task_1(&content));
    println!("Day15. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> String {
    let sum = content.split("\n").map(|x| to_dec(x.trim())).sum();
    println!("sum: {sum}");
    to_sfnau(sum)
}

fn task_2(content: &str) -> i64 {
    todo!()
}

fn to_dec(line: &str) -> i64 {
    let mut ans = 0;
    const mult: i64 = 5;

    for ch in line.chars() {
        let i = match ch {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Unexpected char: '{ch}'"),
        };
        ans *= mult;
        ans += i;
    }

    ans
}

fn to_sfnau(n: i64) -> String {
    const mult: i64 = 5;
    const offset: i64 = 2;

    if n < 0 {
        panic!("Negative numbers not supported");
    }
    if n == 0 {
        return "0".into();
    }

    let mut s: Vec<char> = Vec::new();
    let mut n = n;
    while n > 0 {
        let mut r = n % mult;
        if r > (mult - offset - 1) {
            r = r - mult;
        }

        s.push(match r {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        });

        n -= r;
        n /= mult;
    }

    s.reverse();
    String::from_iter(s.iter())
}
