use std::{
    collections::{HashMap, HashSet},
    env, fs,
    ops::Add,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day21. Task1: {}", task_1(&content));
    println!("Day21. Task2: {}", task_2(&content));
}

fn task_2(content: &str) -> usize {
    const n_rounds: usize = 10;

    let north = Vec2 { x: 0, y: -1 };
    let east = Vec2 { x: 1, y: 0 };
    let south = Vec2 { x: 0, y: 1 };
    let west = Vec2 { x: -1, y: 0 };

    let mut poses = parse_elves(content);

    let printer = |poses: &HashSet<Vec2>| {
        let x_max = poses.iter().map(|v| v.x).max().unwrap();
        let x_min = poses.iter().map(|v| v.x).min().unwrap();
        let y_max = poses.iter().map(|v| v.y).max().unwrap();
        let y_min = poses.iter().map(|v| v.y).min().unwrap();

        // print
        for r in y_min..=y_max {
            print!("{:3}  ", r);

            for c in x_min..=x_max {
                let p = Vec2 { x: c, y: r };
                let ch = if poses.contains(&p) { '#' } else { '.' };
                print!("{}", ch);
            }
            println!("");
        }
        println!("-----------------------------------------------------------------------");
    };

    let mut last_round = 0;
    for round in 0..1000000 {
        last_round = round;
        // making proposes
        let mut move2count: HashMap<Vec2, usize> = HashMap::new();
        let mut proposedMove: HashMap<Vec2, Vec2> = HashMap::new();
        for &pos in &poses {
            let n_pos = pos + north;
            let e_pos = pos + east;
            let s_pos = pos + south;
            let w_pos = pos + west;

            let n = poses.contains(&n_pos);
            let ne = poses.contains(&(pos + north + east));
            let e = poses.contains(&e_pos);
            let se = poses.contains(&(pos + south + east));
            let s = poses.contains(&s_pos);
            let sw = poses.contains(&(pos + south + west));
            let w = poses.contains(&w_pos);
            let nw = poses.contains(&(pos + north + west));

            let has_neibs = n || ne || e || se || s || sw || w || nw;
            if !has_neibs {
                continue;
            }

            let mut apply_if_true = |cond, move_pos| {
                if cond {
                    *move2count.entry(move_pos).or_insert(0) += 1;
                    proposedMove.insert(pos, move_pos);
                    return true;
                }
                return false;
            };

            let choices = vec![
                (!(nw || n || ne), n_pos),
                (!(se || s || sw), s_pos),
                (!(nw || w || sw), w_pos),
                (!(ne || e || se), e_pos),
            ];

            for i in 0..choices.len() {
                let idx = (round + i) % choices.len();
                let choice = choices[idx];
                if apply_if_true(choice.0, choice.1) {
                    break;
                }
            }
        }

        if proposedMove.is_empty() {
            break;
        }

        let mut new_poses = HashSet::new();
        for &pos in &poses {
            if let Some(mv_pos) = proposedMove.get(&pos) {
                if *move2count.get(mv_pos).unwrap() == 1 {
                    new_poses.insert(*mv_pos);
                } else {
                    new_poses.insert(pos);
                }
            } else {
                new_poses.insert(pos);
            }
        }

        poses = new_poses;

        // println!("End of round {}", round + 1);
        // printer(&poses);
    }
    last_round + 1
}

fn task_1(content: &str) -> i32 {
    const n_rounds: usize = 10;

    let north = Vec2 { x: 0, y: -1 };
    let east = Vec2 { x: 1, y: 0 };
    let south = Vec2 { x: 0, y: 1 };
    let west = Vec2 { x: -1, y: 0 };

    let mut poses = parse_elves(content);

    let printer = |poses: &HashSet<Vec2>| {
        let x_max = poses.iter().map(|v| v.x).max().unwrap();
        let x_min = poses.iter().map(|v| v.x).min().unwrap();
        let y_max = poses.iter().map(|v| v.y).max().unwrap();
        let y_min = poses.iter().map(|v| v.y).min().unwrap();

        // print
        for r in y_min..=y_max {
            print!("{:3}  ", r);

            for c in x_min..=x_max {
                let p = Vec2 { x: c, y: r };
                let ch = if poses.contains(&p) { '#' } else { '.' };
                print!("{}", ch);
            }
            println!("");
        }
        println!("-----------------------------------------------------------------------");
    };

    for round in 0..n_rounds {
        // making proposes
        let mut move2count: HashMap<Vec2, usize> = HashMap::new();
        let mut proposedMove: HashMap<Vec2, Vec2> = HashMap::new();
        for &pos in &poses {
            let n_pos = pos + north;
            let e_pos = pos + east;
            let s_pos = pos + south;
            let w_pos = pos + west;

            let n = poses.contains(&n_pos);
            let ne = poses.contains(&(pos + north + east));
            let e = poses.contains(&e_pos);
            let se = poses.contains(&(pos + south + east));
            let s = poses.contains(&s_pos);
            let sw = poses.contains(&(pos + south + west));
            let w = poses.contains(&w_pos);
            let nw = poses.contains(&(pos + north + west));

            let has_neibs = n || ne || e || se || s || sw || w || nw;
            if !has_neibs {
                continue;
            }

            let mut apply_if_true = |cond, move_pos| {
                if cond {
                    *move2count.entry(move_pos).or_insert(0) += 1;
                    proposedMove.insert(pos, move_pos);
                    return true;
                }
                return false;
            };

            let choices = vec![
                (!(nw || n || ne), n_pos),
                (!(se || s || sw), s_pos),
                (!(nw || w || sw), w_pos),
                (!(ne || e || se), e_pos),
            ];

            for i in 0..choices.len() {
                let idx = (round + i) % choices.len();
                let choice = choices[idx];
                if apply_if_true(choice.0, choice.1) {
                    break;
                }
            }
        }

        let mut new_poses = HashSet::new();
        for &pos in &poses {
            if let Some(mv_pos) = proposedMove.get(&pos) {
                if *move2count.get(mv_pos).unwrap() == 1 {
                    new_poses.insert(*mv_pos);
                } else {
                    new_poses.insert(pos);
                }
            } else {
                new_poses.insert(pos);
            }
        }

        poses = new_poses;

        // println!("End of round {}", round + 1);
        // printer(&poses);
    }

    let x_max = poses.iter().map(|v| v.x).max().unwrap();
    let x_min = poses.iter().map(|v| v.x).min().unwrap();
    let y_max = poses.iter().map(|v| v.y).max().unwrap();
    let y_min = poses.iter().map(|v| v.y).min().unwrap();

    (x_max - x_min + 1) * (y_max - y_min + 1) - (poses.len() as i32)
}

fn parse_elves(content: &str) -> HashSet<Vec2> {
    let mut poses = HashSet::new();

    for (y, line) in content.split("\n").enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                poses.insert(Vec2 {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    poses
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
