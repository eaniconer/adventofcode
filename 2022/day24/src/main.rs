use std::{
    collections::{HashSet, VecDeque},
    env, fs, io,
    ops::{Add, AddAssign},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day24. Task1: {}", task_1(&content));
    println!("Day24. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> u32 {
    let mut blizzards = Vec::new();

    let mut height = 0;
    let mut width = 0;

    for (r, line) in content.split("\n").enumerate() {
        height = r;
        width = line.trim().len();
        let y = r as i32;
        for (c, ch) in line.trim().chars().enumerate() {
            if ch == '.' || ch == '#' {
                continue;
            }
            let x = c as i32;
            blizzards.push(Blizzard::from_char(Vec2 { x, y }, ch));
        }
    }
    height += 1;

    let wrap = |pos: Vec2| {
        if pos.x == 0 {
            return Vec2 {
                x: (width as i32) - 2,
                ..pos
            };
        }
        if pos.x == (width as i32) - 1 {
            return Vec2 { x: 1, ..pos };
        }
        if pos.y == 0 {
            return Vec2 {
                y: (height as i32) - 2,
                ..pos
            };
        }
        if pos.y == (height as i32) - 1 {
            return Vec2 { y: 1, ..pos };
        }
        pos
    };

    let is_on_field = |pos: Vec2| {
        pos.x > 0 && pos.x < (width as i32) - 1 && pos.y > 0 && pos.y < (height as i32) - 1
    };

    let dirs = vec![
        Vec2 { x: 1, y: 0 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: 1 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: 0, y: 0 },
    ];

    let height = height as i32;
    let width = width as i32;

    println!("Blizzard map h={}, w={} parsed", height, width);

    let start_pos = Vec2 { x: 1, y: 0 };
    let end_pos = Vec2 {
        x: width - 2,
        y: height - 1,
    };

    let mut e_poses = HashSet::new();
    e_poses.insert(start_pos);

    let mut T = 0;
    'outer: loop {
        // let mut buffer = String::new();
        // io::stdin().read_line(&mut buffer);

        T += 1;

        println!("[T={:3}], states: {}", T, e_poses.len());
        if e_poses.len() == 0 {
            panic!("No way exists");
        }

        let mut pos_with_blizzard = HashSet::new();
        for blizzard in &mut blizzards {
            blizzard.pos += blizzard.dir;
            blizzard.pos = wrap(blizzard.pos);
            pos_with_blizzard.insert(blizzard.pos);
        }

        let mut e_nxt_poses = HashSet::new();

        for &e_pos in &e_poses {
            for &dir in &dirs {
                let nxt_pos = e_pos + dir;

                if nxt_pos == end_pos {
                    break 'outer;
                }

                if nxt_pos == start_pos {
                    e_nxt_poses.insert(nxt_pos);
                    continue;
                }

                if !is_on_field(nxt_pos) {
                    continue;
                }

                if pos_with_blizzard.contains(&nxt_pos) {
                    continue;
                }

                e_nxt_poses.insert(nxt_pos);
            }
        }

        e_poses = e_nxt_poses;

        // for y in 0..height {
        //     for x in 0..width {
        //         let p = Vec2{x, y};
        //         if !is_on_field(p) {
        //             print!("#");
        //             continue;
        //         }
        //         if pos_with_blizzard.contains(&p) {
        //             print!("B");
        //             continue;
        //         }
        //         if e_poses.contains(&p) {
        //             print!("E");
        //             continue;
        //         }
        //         print!(".");
        //     }
        //     println!();
        // }
        // println!("\n----------------------------------------");
    }

    T
}

fn task_2(content: &str) -> u32 {
    let mut blizzards = Vec::new();

    let mut height = 0;
    let mut width = 0;

    for (r, line) in content.split("\n").enumerate() {
        height = r;
        width = line.trim().len();
        let y = r as i32;
        for (c, ch) in line.trim().chars().enumerate() {
            if ch == '.' || ch == '#' {
                continue;
            }
            let x = c as i32;
            blizzards.push(Blizzard::from_char(Vec2 { x, y }, ch));
        }
    }
    height += 1;

    let wrap = |pos: Vec2| {
        if pos.x == 0 {
            return Vec2 {
                x: (width as i32) - 2,
                ..pos
            };
        }
        if pos.x == (width as i32) - 1 {
            return Vec2 { x: 1, ..pos };
        }
        if pos.y == 0 {
            return Vec2 {
                y: (height as i32) - 2,
                ..pos
            };
        }
        if pos.y == (height as i32) - 1 {
            return Vec2 { y: 1, ..pos };
        }
        pos
    };

    let is_on_field = |pos: Vec2| {
        pos.x > 0 && pos.x < (width as i32) - 1 && pos.y > 0 && pos.y < (height as i32) - 1
    };

    let dirs = vec![
        Vec2 { x: 1, y: 0 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: 1 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: 0, y: 0 },
    ];

    let height = height as i32;
    let width = width as i32;

    println!("Blizzard map h={}, w={} parsed", height, width);

    let start_pos = Vec2 { x: 1, y: 0 };
    let end_pos = Vec2 {
        x: width - 2,
        y: height - 1,
    };

    let mut target_queue = VecDeque::new();
    target_queue.push_back(end_pos);
    target_queue.push_back(start_pos);
    target_queue.push_back(end_pos);

    let mut e_poses = HashSet::new();
    e_poses.insert(start_pos);

    let mut T = 0;
    'outer: while !target_queue.is_empty() {
        // let mut buffer = String::new();
        // io::stdin().read_line(&mut buffer);

        T += 1;

        println!("[T={:3}], states: {}", T, e_poses.len());
        if e_poses.len() == 0 {
            panic!("No way exists");
        }

        let mut pos_with_blizzard = HashSet::new();
        for blizzard in &mut blizzards {
            blizzard.pos += blizzard.dir;
            blizzard.pos = wrap(blizzard.pos);
            pos_with_blizzard.insert(blizzard.pos);
        }

        let mut e_nxt_poses = HashSet::new();

        for &e_pos in &e_poses {
            for &dir in &dirs {
                let nxt_pos = e_pos + dir;

                if nxt_pos == *target_queue.front().unwrap() {
                    target_queue.pop_front();
                    e_poses.clear();
                    e_poses.insert(nxt_pos);
                    continue 'outer;
                }

                if nxt_pos == start_pos || nxt_pos == end_pos {
                    e_nxt_poses.insert(nxt_pos);
                    continue;
                }

                if !is_on_field(nxt_pos) {
                    continue;
                }

                if pos_with_blizzard.contains(&nxt_pos) {
                    continue;
                }

                e_nxt_poses.insert(nxt_pos);
            }
        }

        e_poses = e_nxt_poses;

        // for y in 0..height {
        //     for x in 0..width {
        //         let p = Vec2{x, y};
        //         if !is_on_field(p) {
        //             print!("#");
        //             continue;
        //         }
        //         if pos_with_blizzard.contains(&p) {
        //             print!("B");
        //             continue;
        //         }
        //         if e_poses.contains(&p) {
        //             print!("E");
        //             continue;
        //         }
        //         print!(".");
        //     }
        //     println!();
        // }
        // println!("\n----------------------------------------");
    }

    T
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

struct Blizzard {
    pos: Vec2,
    dir: Vec2,
}

impl Blizzard {
    fn from_char(pos: Vec2, ch: char) -> Self {
        Self {
            pos,
            dir: match ch {
                '<' => Vec2 { x: -1, y: 0 },
                '>' => Vec2 { x: 1, y: 0 },
                'v' => Vec2 { x: 0, y: 1 },
                '^' => Vec2 { x: 0, y: -1 },
                _ => panic!("Unexpected ch '{}'", ch),
            },
        }
    }
}
