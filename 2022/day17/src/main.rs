use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day17. Task1: {}", task_1(&content));
    println!("Day17. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> usize {
    let controller = Controller::new(content);
    let rock_producer = CirclularRockProducer::new(configure_rocks());

    let mut game = Game::new(7, 3, 2, controller, rock_producer);

    game.play(2022);

    // game.print();

    game.tower_height
}

fn task_2(content: &str) -> u64 {
    let controller = Controller::new(content);

    let rocks = configure_rocks();
    let rock_count = rocks.len();
    let rock_producer = CirclularRockProducer::new(rocks);

    let mut game = Game::new(7, 3, 2, controller, rock_producer);

    // maket the game stable
    let mut rock_fallen = rock_count * 1000;
    game.play(rock_fallen);

    let mut fingerprint2fallenrock: HashMap<i64, Vec<(usize, usize)>> = HashMap::new();

    'main: loop {
        game.play(rock_count);
        rock_fallen += rock_count;
        let fingerprint = game.make_fingerprint();

        let v = fingerprint2fallenrock
            .entry(fingerprint)
            .or_insert(Vec::new());

        v.push((rock_fallen, game.tower_height));

        if v.len() > 9 {
            let step_diff = v[1].0 - v[0].0;
            let height_diff = v[1].1 - v[0].1;

            for i in 2..v.len() {
                if v[i].0 - v[i - 1].0 != step_diff {
                    continue 'main;
                }
                if v[i].1 - v[i - 1].1 != height_diff {
                    continue 'main;
                }
            }
            // test on cycle passed

            let mut tower_height = game.tower_height as u64;
            let mut rock_to_fall = 1_000_000_000_000 - (rock_fallen as u64);

            tower_height += (rock_to_fall / (step_diff as u64)) * (height_diff as u64);
            rock_to_fall = rock_to_fall % (step_diff as u64);

            let old_height = game.tower_height;
            game.play(rock_to_fall as usize);
            tower_height += (game.tower_height - old_height) as u64;
            return tower_height;
        }
    }
    unreachable!();
}

fn configure_rocks() -> Vec<Rock> {
    let mut rocks = Vec::new();
    rocks.reserve(5);

    rocks.push(Rock::new("####".to_string(), 4, 1));
    rocks.push(Rock::new(".#.###.#.".to_string(), 3, 3));
    rocks.push(Rock::new("..#..####".to_string(), 3, 3));
    rocks.push(Rock::new("####".to_string(), 1, 4));
    rocks.push(Rock::new("####".to_string(), 2, 2));

    rocks
}

enum Action {
    Right,
    Left,
}

struct Controller<'a> {
    pattern: &'a str,
    nxt_idx: usize,
}

impl<'a> Controller<'a> {
    fn new(pattern: &'a str) -> Self {
        Self {
            pattern: pattern.trim(),
            nxt_idx: 0,
        }
    }

    fn next_action(&mut self) -> Action {
        let action = match self.pattern.chars().nth(self.nxt_idx) {
            Some(raw_action) => match raw_action {
                '<' => Action::Left,
                '>' => Action::Right,
                _ => panic!("Unknown action: '{}'", raw_action),
            },
            _ => panic!("Cannot get raw action at index {}", self.nxt_idx),
        };
        self.nxt_idx = (self.nxt_idx + 1) % self.pattern.len();
        action
    }
}

struct CirclularRockProducer {
    rocks: Vec<Rock>,
    nxt_idx: usize,
}

#[derive(Clone)]
struct Rock {
    content: String,
    width: usize,
    height: usize,
}

impl Rock {
    fn new(content: String, width: usize, height: usize) -> Self {
        assert!(width * height == content.len());
        Self {
            content,
            width,
            height,
        }
    }

    fn cell_type(&self, rel_p: Point) -> CellType {
        let idx = (rel_p.y as usize) * self.width + (rel_p.x as usize);
        match self.content.chars().nth(idx).unwrap() {
            '.' => CellType::Air,
            '#' => CellType::Rock,
            c => panic!("Unexpected cell '{}' in rock", c),
        }
    }
}

impl CirclularRockProducer {
    fn new(rocks: Vec<Rock>) -> Self {
        Self { rocks, nxt_idx: 0 }
    }

    fn next_rock(&mut self) -> Rock {
        let nxt_rock = self.rocks[self.nxt_idx].clone();
        self.nxt_idx = (self.nxt_idx + 1) % self.rocks.len();
        nxt_rock
    }
}

struct Game<'a> {
    field: Vec<bool>,
    field_width: usize,
    tower_height: usize,

    spawn_height: usize,
    spawn_x_offset: usize,

    controller: Controller<'a>,
    rock_producer: CirclularRockProducer,
}

impl<'a> Game<'a> {
    fn new(
        field_width: usize,
        spawn_height: usize,
        spawn_x_offset: usize,
        controller: Controller<'a>,
        rock_producer: CirclularRockProducer,
    ) -> Self {
        Self {
            field: Vec::new(),
            tower_height: 0,
            field_width,
            spawn_height,
            spawn_x_offset,
            controller,
            rock_producer,
        }
    }

    fn _in_bounds(&self, p: Point) -> bool {
        p.y >= 0 && p.x >= 0 && p.x < (self.field_width as i32)
    }

    fn _field_index(&self, p: Point) -> usize {
        if !self._in_bounds(p) {
            panic!("Point {:?} is out of bound", p);
        }
        (p.y as usize) * self.field_width + (p.x as usize)
    }

    fn _cell_type(&self, p: Point) -> CellType {
        let idx = self._field_index(p);
        if *self.field.get(idx).unwrap_or(&false) {
            CellType::Rock
        } else {
            CellType::Air
        }
    }

    fn _put_rock_at(&mut self, p: Point) {
        let idx = self._field_index(p);
        if idx >= self.field.len() {
            self.field.resize(idx + self.field_width * 10, false)
        }

        assert!(!self.field[idx]);
        self.field[idx] = true;

        let h = (p.y + 1) as usize;
        if h > self.tower_height {
            self.tower_height = h;
        }
    }

    fn _spawn_point(&self) -> Point {
        Point {
            x: self.spawn_x_offset as i32,
            y: (self.tower_height + self.spawn_height) as i32,
        }
    }

    fn play(&mut self, n_rocks: usize) {
        for _ in 0..n_rocks {
            let rock = FallingRock::new(self.rock_producer.next_rock(), self._spawn_point());
            self._play_rock(rock);
        }
    }

    fn make_fingerprint(&self) -> i64 {
        let mut fingerprint: i64 = 0;
        let from_y = self.tower_height - 1;

        let end_idx = self._field_index(Point {
            x: 0,
            y: self.tower_height as i32,
        });
        let start_idx = end_idx - 60;

        for i in start_idx..end_idx {
            if *self.field.get(i).unwrap_or(&false) {
                fingerprint += (1 << (i - start_idx));
            }
        }
        fingerprint
    }

    fn print(&self) {
        let from_y = self.tower_height + 3;
        for i in 0..from_y {
            let y = (from_y - 1 - i) as i32;
            print!("[{:3}] |", y);
            for x in 0..self.field_width {
                let x = x as i32;
                print!(
                    "{}",
                    match self._cell_type(Point { x, y }) {
                        CellType::Air => '.',
                        CellType::Rock => '#',
                    }
                );
            }
            println!("|");
        }
        println!("[   ] +{}+", str::repeat("-", self.field_width));
    }

    fn _play_rock(&mut self, rock: FallingRock) {
        let mut rock = rock;

        loop {
            // LR-action
            let action = self.controller.next_action();
            let dx = match action {
                Action::Right => 1,
                Action::Left => -1,
            };

            rock.anchor.x += dx;
            if !self._is_valid_rock(&rock) {
                rock.anchor.x -= dx;
            }

            // move-down
            rock.anchor.y -= 1;
            if !self._is_valid_rock(&rock) {
                rock.anchor.y += 1;
                self._freeze_rock(&rock);
                break;
            }
        }
    }

    fn _is_valid_rock(&self, rock: &FallingRock) -> bool {
        let anchor = rock.anchor;
        let rock = &rock.rock;
        for r in 0..rock.height {
            let r = r as i32;
            for c in 0..rock.width {
                let c = c as i32;

                if rock.cell_type(Point {
                    x: c,
                    y: (rock.height as i32) - r - 1,
                }) == CellType::Rock
                {
                    let field_point = Point {
                        x: anchor.x + c,
                        y: anchor.y + r,
                    };

                    if !self._in_bounds(field_point) {
                        return false;
                    }

                    if self._cell_type(field_point) == CellType::Rock {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn _freeze_rock(&mut self, rock: &FallingRock) {
        // todo: get rid of duplicate
        let anchor = rock.anchor;
        let rock = &rock.rock;
        for r in 0..rock.height {
            let r = r as i32;
            for c in 0..rock.width {
                let c = c as i32;
                if rock.cell_type(Point {
                    x: c,
                    y: (rock.height as i32) - r - 1,
                }) == CellType::Rock
                {
                    let field_point = Point {
                        x: anchor.x + c,
                        y: anchor.y + r,
                    };
                    self._put_rock_at(field_point);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq)]
enum CellType {
    Air,
    Rock,
}

struct FallingRock {
    rock: Rock,
    anchor: Point,
}

impl FallingRock {
    fn new(rock: Rock, anchor: Point) -> Self {
        Self { rock, anchor }
    }
}
