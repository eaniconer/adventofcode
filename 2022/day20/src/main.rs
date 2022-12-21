use std::{collections::HashMap, env, fs, num};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    // println!("Day15. Task1: {}", task_1(&content));
    println!("Day15. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> i64 {
    let mut numbers: Vec<_> = content
        .split("\n")
        .map(|x| x.parse().unwrap())
        .enumerate()
        .map(|(id, n)| Item { id, n })
        .collect();

    let mut id2idx = HashMap::new();
    for (idx, n) in numbers.iter().enumerate() {
        id2idx.insert(n.id, idx);
    }

    let mut next_id = 0;
    while !id2idx.is_empty() {
        let idx = id2idx.remove(&next_id).unwrap();
        next_id += 1;

        let item = numbers[idx];

        let mut shift = item.n % (numbers.len() as i64 - 1);

        if shift == 0 {
            continue;
        }

        let dir = shift / shift.abs();
        shift *= dir;

        let mut cur_idx = idx;
        // println!("{} moves with shift {}", item.n, shift);

        while shift > 0 {
            let nxt_idx = if cur_idx == 0 && dir < 0 {
                numbers.len() - 1
            } else {
                (((cur_idx as i64) + dir) as usize) % numbers.len()
            };

            shift -= 1;

            numbers.swap(cur_idx, nxt_idx);

            if let Some(v) = id2idx.get_mut(&numbers[cur_idx].id) {
                *v = cur_idx;
            }

            cur_idx = nxt_idx;
        }
    }

    let zero_idx = numbers.iter().position(|item| item.n == 0).unwrap();

    let n1 = numbers[(zero_idx + 1000) % numbers.len()];
    let n2 = numbers[(zero_idx + 2000) % numbers.len()];
    let n3 = numbers[(zero_idx + 3000) % numbers.len()];
    println!("{} + {} + {}", n1.n, n2.n, n3.n);

    n1.n + n2.n + n3.n
}

fn task_2(content: &str) -> i64 {
    const decription_key: i64 = 811589153;
    const n_pass: usize = 10;

    let mut numbers: Vec<_> = content
        .split("\n")
        .map(|x| x.parse::<i64>().unwrap())
        .enumerate()
        .map(|(id, n)| Item {
            id,
            n: n * decription_key,
        })
        .collect();

    let mut id2idx = HashMap::new();
    for (idx, n) in numbers.iter().enumerate() {
        id2idx.insert(n.id, idx);
    }

    for round in 0..n_pass {
        let mut round_id2idx = id2idx.clone();
        let mut next_id = 0;
        while !round_id2idx.is_empty() {
            let idx = round_id2idx.remove(&next_id).unwrap();
            next_id += 1;

            let item = numbers[idx];

            let mut shift = item.n % (numbers.len() as i64 - 1);

            if shift == 0 {
                continue;
            }

            let dir = shift / shift.abs();
            shift *= dir;

            let mut cur_idx = idx;
            // println!("{} moves with shift {}", item.n, shift);

            while shift > 0 {
                let nxt_idx = if cur_idx == 0 && dir < 0 {
                    numbers.len() - 1
                } else {
                    (((cur_idx as i64) + dir) as usize) % numbers.len()
                };

                shift -= 1;

                numbers.swap(cur_idx, nxt_idx);

                id2idx.insert(numbers[cur_idx].id, cur_idx);
                id2idx.insert(numbers[nxt_idx].id, nxt_idx);
                if let Some(v) = round_id2idx.get_mut(&numbers[cur_idx].id) {
                    *v = cur_idx;
                }

                cur_idx = nxt_idx;
            }
        }

        // print!("After round {}: ", round + 1);
        // for item in &numbers {
        //     print!("{} ", item.n);
        // }
        // println!("\n--");
    }

    let zero_idx = numbers.iter().position(|item| item.n == 0).unwrap();

    let n1 = numbers[(zero_idx + 1000) % numbers.len()];
    let n2 = numbers[(zero_idx + 2000) % numbers.len()];
    let n3 = numbers[(zero_idx + 3000) % numbers.len()];
    println!("{} + {} + {}", n1.n, n2.n, n3.n);

    n1.n + n2.n + n3.n
}

#[derive(Debug, Clone, Copy)]
struct Item {
    id: usize,
    n: i64,
}
