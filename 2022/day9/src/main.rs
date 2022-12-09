use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Debug,
    fs, io,
    ops::{Add, AddAssign},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day9. Task1: {}", task_1(&content));
    println!("Day9. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> usize {
    let moves = parse_moves(content);

    let mut visited = HashSet::new();

    let mut h_pos = Point::new(0, 0);
    let mut t_pos = h_pos;
    let mut layout = Layout::Same;

    visited.insert(t_pos);

    for mv in moves {
        for _ in 0..mv.n {
            h_pos += mv.dir;
            match layout {
                Layout::Same => {
                    layout = Layout::Queue(mv.dir);
                }
                Layout::Queue(qdir) => {
                    if mv.dir == qdir {
                        t_pos += mv.dir;
                    } else if mv.dir == qdir.rot180() {
                        layout = Layout::Same;
                    } else if mv.dir == qdir.rot90() {
                        layout = Layout::Diag(qdir.rot90());
                    } else {
                        layout = Layout::Diag(qdir);
                    }
                }
                Layout::Diag(qdir) => {
                    if mv.dir == qdir {
                        t_pos += mv.dir + mv.dir.rot270();
                        layout = Layout::Queue(qdir);
                    } else if mv.dir == qdir.rot90() {
                        layout = Layout::Queue(qdir);
                    } else if mv.dir == qdir.rot180() {
                        layout = Layout::Queue(qdir.rot270())
                    } else {
                        t_pos += mv.dir + mv.dir.rot90();
                        layout = Layout::Queue(mv.dir);
                    }
                }
            }
            visited.insert(t_pos);
        }
    }

    visited.len()
}

struct Knot {
    pos: Point,
    layout: Layout,
}

impl Debug for Knot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({:?})", self.layout).as_str())
    }
}

struct Rope {
    head: Point,
    tail: Vec<Knot>,
}

impl Rope {
    fn new() -> Rope {
        let mut rope = Rope {
            head: Point::zero(),
            tail: Vec::new(),
        };
        for _ in 0..9 {
            rope.tail.push(Knot {
                pos: Point::zero(),
                layout: Layout::Same,
            });
        }
        return rope;
    }

    fn last_knot(&self) -> Point {
        self.tail.last().unwrap().pos
    }

    fn print(&self) {
        println!("({:?}) - {:?}", self.head, self.tail);
    }

    fn print_field(&self) {
        let mut hm = HashMap::new();
        hm.insert(self.head, 'H');
        for (idx, knot) in self.tail.iter().enumerate() {
            if hm.contains_key(&knot.pos) {
                continue;
            }
            hm.insert(knot.pos, (idx + 1).to_string().chars().next().unwrap());
        }

        for r in 0..50 {
            for c in 0..50 {
                let x = c - 25;
                let y = r - 25;

                if let Some(mark) = hm.get(&Point::new(x, y)) {
                    print!("{}", mark);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }

    fn _on_front_knot_moved(mut self, front_knot_step: Point, knot_idx: usize) -> Rope {
        if knot_idx == self.tail.len() {
            return self;
        }
        let knot = &mut self.tail[knot_idx];
        match knot.layout {
            Layout::Same => {
                if front_knot_step.x == 0 || front_knot_step.y == 0 {
                    knot.layout = Layout::Queue(front_knot_step);
                } else {
                    let x = front_knot_step.x;
                    let y = front_knot_step.y;
                    knot.layout = Layout::Diag(Point::new((x + y) / 2, (-x + y) / 2));
                }
            }
            Layout::Queue(qdir) => {
                if front_knot_step == qdir.n() {
                    knot.pos += front_knot_step;
                    return self._on_front_knot_moved(front_knot_step, knot_idx + 1);
                } else if front_knot_step == qdir.s() {
                    knot.layout = Layout::Same;
                } else if front_knot_step == qdir.w() {
                    knot.layout = Layout::Diag(qdir.w());
                } else if front_knot_step == qdir.e() {
                    knot.layout = Layout::Diag(qdir.n());
                } else if front_knot_step == qdir.ne() {
                    knot.pos += front_knot_step;
                    return self._on_front_knot_moved(front_knot_step, knot_idx + 1);
                } else if front_knot_step == qdir.nw() {
                    knot.pos += front_knot_step;
                    return self._on_front_knot_moved(front_knot_step, knot_idx + 1);
                } else if front_knot_step == qdir.sw() {
                    knot.layout = Layout::Queue(qdir.w());
                } else if front_knot_step == qdir.se() {
                    knot.layout = Layout::Queue(qdir.e());
                } else {
                    println!("DIR: {:?}", front_knot_step);
                    unreachable!()
                }
            }
            Layout::Diag(qdir) => {
                if front_knot_step == qdir.n() {
                    knot.layout = Layout::Queue(qdir);
                    knot.pos += qdir.ne();
                    return self._on_front_knot_moved(qdir.ne(), knot_idx + 1);
                } else if front_knot_step == qdir.s() {
                    knot.layout = Layout::Queue(qdir.e());
                } else if front_knot_step == qdir.w() {
                    knot.layout = Layout::Queue(qdir.n());
                } else if front_knot_step == qdir.e() {
                    knot.layout = Layout::Queue(qdir.e());
                    knot.pos += qdir.ne();
                    return self._on_front_knot_moved(qdir.ne(), knot_idx + 1);
                } else if front_knot_step == qdir.ne() {
                    knot.pos += qdir.ne();
                    return self._on_front_knot_moved(qdir.ne(), knot_idx + 1);
                } else if front_knot_step == qdir.nw() {
                    knot.layout = Layout::Queue(qdir.n());
                    knot.pos += qdir.n();
                    return self._on_front_knot_moved(qdir.n(), knot_idx + 1);
                } else if front_knot_step == qdir.sw() {
                    knot.layout = Layout::Same;
                } else if front_knot_step == qdir.se() {
                    knot.layout = Layout::Queue(qdir.e());
                    knot.pos += qdir.e();
                    return self._on_front_knot_moved(qdir.e(), knot_idx + 1);
                } else {
                    println!("DIR: {:?}", front_knot_step);
                    unreachable!()
                }
            }
        }
        self
    }

    fn step(mut rope: Rope, dir: Point) -> Rope {
        rope.head += dir;
        Rope::_on_front_knot_moved(rope, dir, 0)
    }
}

fn task_2(content: &str) -> usize {
    let moves = parse_moves(content);

    let mut visited = HashSet::new();
    let mut rope = Rope::new();
    visited.insert(rope.last_knot());

    for mv in moves {
        for _ in 0..mv.n {
            // println!("STEP: {:?}", mv.dir);
            rope = Rope::step(rope, mv.dir);
            visited.insert(rope.last_knot());

            // rope.print_field();
            // rope.print();
            // let mut user_input = String::new();
            // let stdin = io::stdin(); // We get `Stdin` here.
            // stdin.read_line(&mut user_input);
        }
    }

    visited.len()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn zero() -> Point {
        Point::new(0, 0)
    }

    fn rot180(&self) -> Point {
        Point::new(-self.x, -self.y)
    }

    fn rot90(&self) -> Point {
        Point::new(self.y, -self.x)
    }

    fn rot270(&self) -> Point {
        self.rot180().rot90()
    }

    fn n(&self) -> Point {
        *self
    }
    fn s(&self) -> Point {
        self.rot180()
    }
    fn e(&self) -> Point {
        self.rot270()
    }
    fn w(&self) -> Point {
        self.rot90()
    }
    fn ne(&self) -> Point {
        self.n() + self.e()
    }
    fn nw(&self) -> Point {
        self.n() + self.w()
    }
    fn se(&self) -> Point {
        self.s() + self.e()
    }
    fn sw(&self) -> Point {
        self.s() + self.w()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Point {
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }

    type Output = Self;
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({},{})", self.x, self.y).as_str())
        // f.debug_struct("p").field("", &self.x).field("", &self.y).finish()
    }
}

struct Move {
    dir: Point,
    n: u32,
}

#[derive(Debug)]
enum Layout {
    Same,
    Queue(Point),
    Diag(Point),
}

fn parse_moves(content: &str) -> Vec<Move> {
    content.split("\n").map(parse_move).collect()
}

fn parse_move(line: &str) -> Move {
    let mut splitter = line.trim().split(' ');
    let dir = match splitter.next().unwrap() {
        "R" => Point::new(1, 0),
        "L" => Point::new(-1, 0),
        "U" => Point::new(0, -1),
        "D" => Point::new(0, 1),
        _ => panic!("Unexpected move: {}", line),
    };
    let n = splitter.next().unwrap().parse::<u32>().unwrap();
    return Move { dir, n };
}
