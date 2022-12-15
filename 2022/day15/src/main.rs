use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day15. Task1: {}", task_1(&content));
    println!("Day15. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> u32 {
    let pairs: Vec<Pair> = content.split("\n").map(parse_pair).collect();

    let (leftmost, rightmost) = pairs
        .iter()
        .map(|it| (it.sensor.coord.x - it.manh(), it.sensor.coord.x + it.manh()))
        .reduce(|l, r| (l.0.min(r.0), l.1.max(r.1)))
        .unwrap();

    let mut count = 0;

    // let y = 10; // smoke
    let y = 2000000; // main input
    'outer: for x in leftmost..rightmost {
        let p = Point::new(x, y);

        for pair in pairs.iter() {
            if pair.sensor.coord.manh_dist(p) <= pair.manh() && p != pair.beacon.coord {
                count += 1;
                continue 'outer;
            }
        }
    }

    count
}

fn task_2(content: &str) -> i64 {
    let pairs: Vec<Pair> = content.split("\n").map(parse_pair).collect();

    let max_y = 4000000;
    let mut segments = Vec::new();
    for y in 0..max_y + 1 {
        if y % 100000 == 0 {
            println!("Scan y: {}", y);
        }
        segments.clear();
        for pair in pairs.iter() {
            let m = pair.manh();
            let rest = m - (pair.sensor.coord.y - y).abs();
            if rest < 0 {
                continue;
            }
            // print!("Build seg for y={}, sensor=({},{})", y, pair.sensor.coord.x, pair.sensor.coord.y);
            let x = pair.sensor.coord.x;
            let segment = Segment::new((x - rest).max(0), x + rest);
            // println!(". Segment: [{}, {}]", segment.lhs, segment.rhs);
            segments.push(segment);
        }
        segments.sort_by_key(|item| item.lhs);

        let mut r_point: i32 = -1;
        for segment in &segments {
            if segment.lhs == r_point + 2 {
                let ans = ((r_point + 1) as i64) * 4000000;
                return ans + (y as i64);
            }
            assert!(segment.lhs <= r_point + 1);
            r_point = r_point.max(segment.rhs);
        }

        if y == 11 {
            println!("{:?}", segments);
        }

        // intentionally don't check rigth border;
    }

    unreachable!()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Segment {
    lhs: i32,
    rhs: i32,
}

impl Segment {
    fn new(lhs: i32, rhs: i32) -> Self {
        Self { lhs, rhs }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn manh_dist(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Sensor {
    coord: Point,
}

struct Beacon {
    coord: Point,
}

struct Pair {
    sensor: Sensor,
    beacon: Beacon,
}

impl Pair {
    fn new(sensor: Sensor, beacon: Beacon) -> Self {
        Self { sensor, beacon }
    }

    fn manh(&self) -> i32 {
        self.sensor.coord.manh_dist(self.beacon.coord)
    }
}

fn parse_pair(line: &str) -> Pair {
    let mut splitter = line.trim().split(' ');

    splitter.next(); // Skip 'Sensor'
    splitter.next(); // Skip 'at'

    let sx = splitter.next().unwrap();
    let sx: i32 = sx.get(2..sx.len() - 1).unwrap().parse().unwrap();

    let sy = splitter.next().unwrap();
    let sy: i32 = sy.get(2..sy.len() - 1).unwrap().parse().unwrap();

    splitter.next(); // Skip 'closest'
    splitter.next(); // Skip 'beacon'
    splitter.next(); // Skip 'is'
    splitter.next(); // Skip 'at'

    let bx = splitter.next().unwrap();
    let bx: i32 = bx.get(2..bx.len() - 1).unwrap().parse().unwrap();

    let by = splitter.next().unwrap();
    let by: i32 = by.get(2..).unwrap().parse().unwrap();

    Pair {
        sensor: Sensor {
            coord: Point::new(sx, sy),
        },
        beacon: Beacon {
            coord: Point::new(bx, by),
        },
    }
}
