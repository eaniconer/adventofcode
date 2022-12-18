use std::{
    collections::{HashSet, VecDeque},
    env, fs,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day18. Task1: {}", task_1(&content));
    println!("Day18. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> i32 {
    let cubes: HashSet<Point> = content.split("\n").map(parse_point).collect();

    let mut n_sides = 0;
    for cube in &cubes {
        n_sides += 6;

        for neighbor in neighbors(*cube) {
            if cubes.contains(&neighbor) {
                n_sides -= 1;
            }
        }
    }

    n_sides
}

fn task_2(content: &str) -> u32 {
    let cubes: HashSet<Point> = content.split("\n").map(parse_point).collect();

    let mut bbox = BBox::new(*cubes.iter().next().unwrap());
    for cube in &cubes {
        bbox.extend(*cube);
    }

    let mut outside: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();

    bbox.extend(Point {
        x: bbox.min.x - 1,
        y: bbox.min.y - 1,
        z: bbox.min.z - 1,
    });
    bbox.extend(Point {
        x: bbox.max.x + 1,
        y: bbox.max.y + 1,
        z: bbox.max.z + 1,
    });

    outside.insert(bbox.min);
    queue.push_back(bbox.min);

    while let Some(p) = queue.pop_front() {
        for n in neighbors(p) {
            if !bbox.in_bounds(n) {
                continue;
            }
            if cubes.contains(&n) {
                continue;
            }
            if outside.contains(&n) {
                continue;
            }
            outside.insert(n);
            queue.push_back(n);
        }
    }

    let mut n_sides = 0;

    for cube in &cubes {
        n_sides += 6;
        for neighbor in neighbors(*cube) {
            if cubes.contains(&neighbor) {
                n_sides -= 1;
                continue;
            }
            if !outside.contains(&neighbor) {
                n_sides -= 1;
            }
        }
    }

    n_sides
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn parse_point(line: &str) -> Point {
    let mut it = line.trim().split(",").map(|x| x.parse().unwrap());
    Point {
        x: it.next().unwrap(),
        y: it.next().unwrap(),
        z: it.next().unwrap(),
    }
}

fn neighbors(p: Point) -> Vec<Point> {
    let mut ns = Vec::new();
    ns.reserve(6);

    ns.push(Point { x: p.x - 1, ..p });
    ns.push(Point { x: p.x + 1, ..p });

    ns.push(Point { y: p.y - 1, ..p });
    ns.push(Point { y: p.y + 1, ..p });

    ns.push(Point { z: p.z - 1, ..p });
    ns.push(Point { z: p.z + 1, ..p });

    ns
}

#[derive(Debug)]
struct BBox {
    min: Point,
    max: Point,
}

impl BBox {
    fn new(p: Point) -> Self {
        Self { min: p, max: p }
    }

    fn extend(&mut self, p: Point) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.min.z = self.min.z.min(p.z);

        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
        self.max.z = self.max.z.max(p.z);
    }

    fn in_bounds(&self, p: Point) -> bool {
        self.min.x <= p.x
            && p.x <= self.max.x
            && self.min.y <= p.y
            && p.y <= self.max.y
            && self.min.z <= p.z
            && p.z <= self.max.z
    }
}
