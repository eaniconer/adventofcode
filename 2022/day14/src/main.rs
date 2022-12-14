use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day14. Task1: {}", task_1(&content));
    println!("Day14. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> u32 {
    let mut canvas = Canvas::new(200, 1000);
    let mut max_y = 0;
    for path in content.trim().split("\n").map(parse_path) {
        max_y = max_y.max(path[0].y);
        for i in 1..path.len() {
            canvas.paint_line(path[i - 1], path[i]);
            max_y = max_y.max(path[i].y);
        }
    }
    let source = Point { x: 500, y: 0 };
    canvas.paint_char_at('+', source);

    // canvas.print_window(Point{x: 493, y: 0}, 11, 11);
    // println!("Max y: {}", max_y);

    let mut units_of_sand = 0;
    'sim: loop {
        let mut p = source;

        loop {
            let p1 = Point::new(p.x, p.y + 1);
            let p2 = Point::new(p.x - 1, p.y + 1);
            let p3 = Point::new(p.x + 1, p.y + 1);
            let mut found = false;
            for pn in vec![p1, p2, p3] {
                if canvas.char(pn) == '.' {
                    p = pn;
                    found = true;
                    break;
                }
            }

            if p.y == max_y {
                break 'sim;
            }

            if !found {
                canvas.paint_char_at('o', p);
                units_of_sand += 1;
                break;
            }
        }

        // canvas.print_window(Point{x: 493, y: 0}, 11, 11);
        // println!("-------------------------");
    }

    return units_of_sand;
}

fn task_2(content: &str) -> usize {
    let w = 1000;
    let mut canvas = Canvas::new(200, w);
    let mut max_y = 0;
    for path in content.trim().split("\n").map(parse_path) {
        max_y = max_y.max(path[0].y);
        for i in 1..path.len() {
            canvas.paint_line(path[i - 1], path[i]);
            max_y = max_y.max(path[i].y);
        }
    }
    let source = Point { x: 500, y: 0 };
    canvas.paint_char_at('+', source);
    canvas.paint_line(
        Point::new(0, max_y + 2),
        Point::new((w as i32) - 1, max_y + 2),
    );

    // canvas.print_window(Point{x: 493, y: 0}, 11, 11);
    // println!("Max y: {}", max_y);

    let mut units_of_sand = 0;
    'sim: loop {
        let mut p = source;

        loop {
            let p1 = Point::new(p.x, p.y + 1);
            let p2 = Point::new(p.x - 1, p.y + 1);
            let p3 = Point::new(p.x + 1, p.y + 1);
            let mut found = false;
            for pn in vec![p1, p2, p3] {
                if canvas.char(pn) == '.' {
                    p = pn;
                    found = true;
                    break;
                }
            }

            if !found {
                units_of_sand += 1;
                let old_ch = canvas.char(p);
                canvas.paint_char_at('o', p);

                if old_ch == '+' {
                    break 'sim;
                }

                break;
            }
        }

        // canvas.print_window(Point{x: 493, y: 0}, 11, 11);
        // println!("-------------------------");
    }

    return units_of_sand;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn parse_path(line: &str) -> Vec<Point> {
    line.trim().split(" -> ").map(parse_point).collect()
}

fn parse_point(line: &str) -> Point {
    let mut it = line.trim().split(",").map(|n| n.parse().unwrap());
    Point {
        x: it.next().unwrap(),
        y: it.next().unwrap(),
    }
}

struct Canvas {
    h: usize,
    w: usize,
    buf: Vec<char>,
}

fn norm(n: i32) -> i32 {
    if n == 0 {
        n
    } else {
        n / n.abs()
    }
}

impl Canvas {
    fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            buf: vec!['.'; h * w],
        }
    }

    fn _buf_idx(&self, p: Point) -> usize {
        let idx = (p.y as usize) * self.w + (p.x as usize);
        if idx >= self.buf.len() {
            panic!("{:?} is out of canvas", p);
        }
        idx
    }

    fn paint_char_at(&mut self, ch: char, p: Point) {
        let idx = self._buf_idx(p);
        self.buf[idx] = ch;
    }

    fn paint_line(&mut self, a: Point, b: Point) {
        let dx = norm(b.x - a.x);
        let dy = norm(b.y - a.y);

        if dx * dy > 0 {
            panic!("Only vertical and horizontal lines supported")
        }

        let mut r = a;
        while r != b {
            self.paint_char_at('#', r);
            r.x += dx;
            r.y += dy;
        }
        self.paint_char_at('#', b);
    }

    fn char(&self, p: Point) -> char {
        let idx = self._buf_idx(p);
        return self.buf[idx];
    }

    fn print_window(&self, topleft: Point, h: i32, w: i32) {
        for i in 0..h {
            for j in 0..w {
                let idx = self._buf_idx(Point {
                    x: topleft.x + j,
                    y: topleft.y + i,
                });
                print!("{}", self.buf[idx]);
            }
            println!("");
        }
    }
}
