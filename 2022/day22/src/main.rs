use std::{collections::HashMap, env, fmt::Display, fs, io, ops::Add};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day21. Task1: {}", task_1(&content));
    println!("Day21. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> i32 {
    let (field, content) = parse_field(content);
    let actions = parse_actions(content);

    let x = (field[0].iter().position(|c| *c == Cell::Empty).unwrap()) as i32;
    let y = 0;

    let mut env = Env::new(field, Walker::new(Vec2 { x, y }), actions);

    env.play();

    calc_score_1(&env.walker)
}

fn task_2(content: &str) -> i32 {
    let (field, content) = parse_field(content);
    let actions = parse_actions(content);

    let n_cells: usize = field
        .iter()
        .map(|r| r.iter().filter(|&c| *c != Cell::Out).count())
        .sum();

    const n_faces: usize = 6;
    if n_cells % n_faces != 0 {
        panic!("Such a number of cells cannot be on the surface of cube");
    }
    let n_cells_per_face = n_cells / n_faces;
    println!("Number of cells per face: {}", n_cells_per_face);

    let n_cells_per_edge: usize = (n_cells_per_face as f32).sqrt() as usize;
    if n_cells_per_edge * n_cells_per_edge != n_cells_per_face {
        panic!("The area of a face is not the square of an integer");
    }

    let field_width = field.iter().map(|r| r.len()).max().unwrap();

    if field.len() % n_cells_per_edge != 0 {
        panic!(
            "Map height[={}] must be a multiple of the {}",
            field.len(),
            n_cells_per_edge
        )
    }
    if field_width % n_cells_per_edge != 0 {
        panic!(
            "Map width[={}] must be a multiple of the {}",
            field_width, n_cells_per_edge
        );
    }

    let mut env = Env2::new(field, actions, n_cells_per_edge as i32);

    env.play();

    calc_score_1(&env.walker)
}

type Dir = Vec2;
type Pos = Vec2;

#[derive(Debug, Clone, Copy)]
struct Face {
    pos: Pos, // position on fiels
    orientation: Vec2,
}

impl Face {
    fn no_pos() -> Vec2 {
        Vec2 { x: -1, y: -1 }
    }
}

impl Default for Face {
    fn default() -> Self {
        Self {
            pos: Self::no_pos(),
            orientation: Vec2 { x: 0, y: 0 },
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Cube {
    back: Face,
    left: Face,
    front: Face,
    right: Face,
    top: Face,
    bottom: Face,
}

impl Cube {
    fn rotY90CW(&self) -> Self {
        let mut obj = Self {
            back: self.back,
            left: self.bottom,
            front: self.front,
            right: self.top,
            top: self.left,
            bottom: self.right,
        };

        obj.back.orientation = obj.back.orientation.rotCCW90();

        obj.left.orientation = obj.left.orientation.rotCW90();
        obj.front.orientation = obj.front.orientation.rotCW90();
        obj.right.orientation = obj.right.orientation.rotCW90();
        obj.top.orientation = obj.top.orientation.rotCW90();
        obj.bottom.orientation = obj.bottom.orientation.rotCW90();

        obj
    }

    fn rotX90CW(&self) -> Self {
        let mut obj = Self {
            back: self.top,
            left: self.left,
            front: self.bottom,
            right: self.right,
            top: self.front,
            bottom: self.back,
        };

        obj.back.orientation = obj.back.orientation.rotCW90().rotCW90();
        obj.bottom.orientation = obj.bottom.orientation.rotCW90().rotCW90();
        obj.left.orientation = obj.left.orientation.rotCCW90();
        obj.right.orientation = obj.right.orientation.rotCW90();

        obj
    }

    fn rotX90CCW(&self) -> Self {
        Self { ..*self }.rotX90CW().rotX90CW().rotX90CW()
    }

    fn rotZ90CW(&self) -> Self {
        let mut obj = Self {
            back: self.left,
            left: self.front,
            front: self.right,
            right: self.back,
            top: self.top,
            bottom: self.bottom,
        };

        obj.top.orientation = obj.top.orientation.rotCW90();
        obj.bottom.orientation = obj.bottom.orientation.rotCCW90();

        obj
    }

    fn rotZ90CCW(&self) -> Self {
        Self { ..*self }.rotZ90CW().rotZ90CW().rotZ90CW()
    }

    fn fill(mut self, pos: Vec2, field: &Vec<Vec<Cell>>, step: i32) -> Self {
        if self.front.pos != Face::no_pos() {
            return self;
        }

        self.front.pos = pos;
        self.front.orientation = Vec2 { x: 0, y: -1 };

        let is_on_cube = |pos: Vec2| {
            if pos.y < 0 || (pos.y as usize) >= field.len() {
                return false;
            }
            let row = field.iter().nth(pos.y as usize).unwrap();
            if pos.x < 0 || (pos.x as usize) >= row.len() {
                return false;
            }
            return row[pos.x as usize] != Cell::Out;
        };

        let neib = pos + Vec2 { x: 0, y: -step };
        if is_on_cube(neib) {
            self = self.rotX90CCW();
            self = self.fill(neib, field, step);
            self = self.rotX90CW();
        }

        let neib = pos + Vec2 { x: 0, y: step };
        if is_on_cube(neib) {
            self = self.rotX90CW();
            self = self.fill(neib, field, step);
            self = self.rotX90CCW();
        }

        let neib = pos + Vec2 { x: step, y: 0 };
        if is_on_cube(neib) {
            self = self.rotZ90CW();
            self = self.fill(neib, field, step);
            self = self.rotZ90CCW();
        }

        let neib = pos + Vec2 { x: -step, y: 0 };
        if is_on_cube(neib) {
            self = self.rotZ90CCW();
            self = self.fill(neib, field, step);
            self = self.rotZ90CW();
        }

        self
    }

    fn from_field(field: &Vec<Vec<Cell>>, step: i32) -> Self {
        let x = field[0].iter().position(|c| *c != Cell::Out).unwrap() as i32;
        let pos = Vec2 { x, y: 0 };
        Cube::default().fill(pos, &field, step)
    }
}

struct Env2 {
    field: Vec<Vec<Cell>>,
    cube: Cube,
    step: i32,
    walker: Walker,
    actions: Vec<Action>,
}

impl Env2 {
    fn new(field: Vec<Vec<Cell>>, actions: Vec<Action>, step: i32) -> Self {
        let cube = Cube::from_field(&field, step);

        let x = field[0].iter().position(|c| *c != Cell::Out).unwrap() as i32;
        let walker = Walker::new(Vec2 { x, y: 0 });

        Self {
            field,
            cube,
            step,
            walker,
            actions,
        }
    }

    fn play(&mut self) {
        let mut canvas: Vec<Vec<char>> = Vec::new();
        for r in &self.field {
            canvas.push(
                r.iter()
                    .map(|c| match *c {
                        Cell::Out => ' ',
                        Cell::Empty => '.',
                        Cell::Wall => '#',
                    })
                    .collect(),
            );
        }

        let n_actions = self.actions.len();
        for i in 0..n_actions {
            // let mut buffer = String::new();
            // io::stdin().read_line(&mut buffer);

            match self.actions[i] {
                Action::Move(steps) => self.move_walker(steps, &mut canvas),
                Action::Rotate90CW => self.walker.dir = self.walker.dir.rotCW90(),
                Action::Rotate90CCW => self.walker.dir = self.walker.dir.rotCCW90(),
            }

            let ch = match self.walker.dir {
                Vec2 { x: 1, y: 0 } => '>',
                Vec2 { x: 0, y: 1 } => 'v',
                Vec2 { x: -1, y: 0 } => '<',
                Vec2 { x: 0, y: -1 } => '^',
                _ => panic!("Unexpected dir: {:?}", self.walker.dir),
            };

            // canvas[self.walker.pos.y as usize][self.walker.pos.x as usize] = ch;
            // for r in &canvas {
            //     for c in r {
            //         print!("{}", *c);
            //     }
            //     println!("");
            // }
            // println!("");
        }
    }

    fn move_walker(&mut self, steps: i32, canvas: &mut Vec<Vec<char>>) {
        for _ in 0..steps {
            // dbg!(self.walker.pos);

            // let mut buffer = String::new();
            // io::stdin().read_line(&mut buffer);

            let mut nxt_pos = self.walker.pos + self.walker.dir;
            let mut nxt_dir = self.walker.dir;

            match self.cell_at_pos(nxt_pos) {
                Cell::Out => {
                    let face_pos = Vec2 {
                        x: (self.walker.pos.x / self.step) * self.step,
                        y: (self.walker.pos.y / self.step) * self.step,
                    };

                    self.rotate_cube(face_pos);

                    // println!("{:#?}", self.cube);

                    let face_x = self.walker.pos.x % self.step;
                    let face_y = self.walker.pos.y % self.step;

                    let mut nxt_pos_on_neib_face = nxt_pos;

                    match self.walker.dir {
                        Vec2 { x: 1, y: 0 } => match self.cube.right.orientation {
                            Vec2 { x: 0, y: -1 } => {
                                nxt_pos_on_neib_face =
                                    self.cube.right.pos + Vec2 { x: 0, y: face_y };
                            }
                            Vec2 { x: -1, y: 0 } => {
                                nxt_pos_on_neib_face = self.cube.right.pos
                                    + Vec2 {
                                        x: self.step - 1 - face_y,
                                        y: 0,
                                    };
                                nxt_dir = Vec2 { x: 0, y: 1 };
                            }
                            Vec2 { x: 1, y: 0 } => {
                                nxt_pos_on_neib_face = self.cube.right.pos
                                    + Vec2 {
                                        x: face_y,
                                        y: self.step - 1,
                                    };
                                nxt_dir = Vec2 { x: 0, y: -1 };
                            }
                            Vec2 { x: 0, y: 1 } => {
                                nxt_pos_on_neib_face = self.cube.right.pos
                                    + Vec2 {
                                        x: self.step - 1,
                                        y: self.step - 1 - face_y,
                                    };
                                nxt_dir = Vec2 { x: -1, y: 0 };
                            }
                            _ => unreachable!(),
                        },
                        Vec2 { x: 0, y: 1 } => match self.cube.bottom.orientation {
                            Vec2 { x: 0, y: -1 } => {
                                nxt_pos_on_neib_face =
                                    self.cube.bottom.pos + Vec2 { x: face_x, y: 0 };
                            }
                            Vec2 { x: -1, y: 0 } => {
                                nxt_pos_on_neib_face = self.cube.bottom.pos
                                    + Vec2 {
                                        x: self.step - 1,
                                        y: face_x,
                                    };
                                nxt_dir = Vec2 { x: -1, y: 0 };
                            }
                            Vec2 { x: 1, y: 0 } => {
                                nxt_pos_on_neib_face = self.cube.bottom.pos
                                    + Vec2 {
                                        x: 0,
                                        y: self.step - 1 - face_x,
                                    };
                                nxt_dir = Vec2 { x: 1, y: 0 };
                            }
                            Vec2 { x: 0, y: 1 } => {
                                nxt_pos_on_neib_face = self.cube.bottom.pos
                                    + Vec2 {
                                        x: self.step - 1 - face_x,
                                        y: self.step - 1,
                                    };
                                nxt_dir = Vec2 { x: 0, y: -1 };
                            }
                            _ => unreachable!(),
                        },
                        Vec2 { x: -1, y: 0 } => match self.cube.left.orientation {
                            Vec2 { x: 0, y: -1 } => {
                                nxt_pos_on_neib_face = self.cube.left.pos
                                    + Vec2 {
                                        x: self.step - 1,
                                        y: face_y,
                                    };
                            }
                            Vec2 { x: -1, y: 0 } => {
                                nxt_pos_on_neib_face = self.cube.left.pos
                                    + Vec2 {
                                        x: self.step - 1 - face_y,
                                        y: self.step - 1,
                                    };
                                nxt_dir = Vec2 { x: 0, y: -1 };
                            }
                            Vec2 { x: 1, y: 0 } => {
                                nxt_pos_on_neib_face =
                                    self.cube.left.pos + Vec2 { x: face_y, y: 0 };
                                nxt_dir = Vec2 { x: 0, y: 1 };
                            }
                            Vec2 { x: 0, y: 1 } => {
                                nxt_pos_on_neib_face = self.cube.left.pos
                                    + Vec2 {
                                        x: 0,
                                        y: self.step - 1 - face_y,
                                    };
                                nxt_dir = Vec2 { x: 1, y: 0 };
                            }
                            _ => unreachable!(),
                        },
                        Vec2 { x: 0, y: -1 } => match self.cube.top.orientation {
                            Vec2 { x: 0, y: -1 } => {
                                nxt_pos_on_neib_face = self.cube.top.pos
                                    + Vec2 {
                                        x: face_x,
                                        y: self.step - 1,
                                    };
                            }
                            Vec2 { x: -1, y: 0 } => {
                                nxt_pos_on_neib_face = self.cube.top.pos + Vec2 { x: 0, y: face_x };
                                nxt_dir = Vec2 { x: 1, y: 0 };
                            }
                            Vec2 { x: 1, y: 0 } => {
                                nxt_pos_on_neib_face = self.cube.top.pos
                                    + Vec2 {
                                        x: self.step - 1,
                                        y: self.step - 1 - face_x,
                                    };
                                nxt_dir = Vec2 { x: -1, y: 0 };
                            }
                            Vec2 { x: 0, y: 1 } => {
                                nxt_pos_on_neib_face = self.cube.top.pos
                                    + Vec2 {
                                        x: self.step - 1 - face_x,
                                        y: 0,
                                    };
                                nxt_dir = Vec2 { x: 0, y: 1 };
                            }
                            _ => unreachable!(),
                        },
                        _ => panic!("Unexpected dir: {:?}", self.walker.dir),
                    }

                    if self.cell_at_pos(nxt_pos_on_neib_face) == Cell::Wall {
                        return;
                    }

                    nxt_pos = nxt_pos_on_neib_face;
                }
                Cell::Empty => (),
                Cell::Wall => return,
            }

            self.walker.pos = nxt_pos;
            self.walker.dir = nxt_dir;

            let ch = match self.walker.dir {
                Vec2 { x: 1, y: 0 } => '>',
                Vec2 { x: 0, y: 1 } => 'v',
                Vec2 { x: -1, y: 0 } => '<',
                Vec2 { x: 0, y: -1 } => '^',
                _ => panic!("Unexpected dir: {:?}", self.walker.dir),
            };
            canvas[self.walker.pos.y as usize][self.walker.pos.x as usize] = ch;
        }
    }

    fn cell_at_pos(&self, pos: Vec2) -> Cell {
        match self.field.iter().nth(pos.y as usize) {
            Some(row) => match row.iter().nth(pos.x as usize) {
                Some(cell) => *cell,
                None => Cell::Out,
            },
            None => Cell::Out,
        }
    }

    fn rotate_cube(&mut self, front_face_pos: Pos) {
        if self.cube.top.pos == front_face_pos {
            self.cube = self.cube.rotX90CCW();
        } else if self.cube.bottom.pos == front_face_pos {
            self.cube = self.cube.rotX90CW();
        } else {
            let mut counter = 0;
            while self.cube.front.pos != front_face_pos {
                self.cube = self.cube.rotZ90CW();
                counter += 1;
                if counter > 4 {
                    panic!("Face not found on cube");
                }
            }
        }

        let up_orientation = Vec2 { x: 0, y: -1 };
        while self.cube.front.orientation != up_orientation {
            self.cube = self.cube.rotY90CW();
        }
    }
}

fn parse_field(content: &str) -> (Vec<Vec<Cell>>, &str) {
    let mut field: Vec<Vec<Cell>> = Vec::new();
    let mut rest: &str = content;

    loop {
        let (part1, part2) = rest.split_once("\n").unwrap();
        rest = part2;

        if part1.trim().is_empty() {
            break;
        }
        field.push(part1.chars().map(Cell::from_char).collect());
    }

    return (field, rest);
}

fn parse_actions(mut line: &str) -> Vec<Action> {
    let mut actions = Vec::new();

    line = line.trim();
    while !line.is_empty() {
        let (action, rest) = parse_action(line);
        line = rest;
        actions.push(action);
    }

    actions
}

fn parse_action(line: &str) -> (Action, &str) {
    if line.starts_with("L") {
        return (Action::Rotate90CCW, line.get(1..).unwrap());
    }
    if line.starts_with("R") {
        return (Action::Rotate90CW, line.get(1..).unwrap());
    }

    let mut i = 0;
    let mut int_line = line;
    while !int_line.is_empty() {
        match int_line.get(..1).unwrap().parse::<i32>() {
            Ok(x) => i = 10 * i + x,
            Err(_) => break,
        }
        int_line = int_line.get(1..).unwrap();
    }

    if i == 0 {
        panic!("Failed to parse int from {}", line);
    }
    (Action::Move(i), int_line)
}

fn calc_score_1(walker: &Walker) -> i32 {
    let dir_score = match walker.dir {
        Vec2 { x: 1, y: 0 } => 0,
        Vec2 { x: 0, y: 1 } => 1,
        Vec2 { x: -1, y: 0 } => 2,
        Vec2 { x: 0, y: -1 } => 3,
        _ => panic!("Unexpected dir: {:?}", walker.dir),
    };
    1000 * (walker.pos.y + 1) + 4 * (walker.pos.x + 1) + dir_score
}

struct Env {
    field: Vec<Vec<Cell>>,
    field_width: usize,

    walker: Walker,

    actions: Vec<Action>,
}

impl Env {
    fn new(field: Vec<Vec<Cell>>, walker: Walker, actions: Vec<Action>) -> Self {
        let field_width = field.iter().map(|r| r.len()).max().unwrap();

        Self {
            field,
            field_width,
            walker,
            actions,
        }
    }

    fn play(&mut self) {
        // let mut canvas: Vec<Vec<char>> = Vec::new();
        // for r in &self.field {
        //     canvas.push(r.iter().map(|c| {
        //         match *c {
        //             Cell::Out => ' ',
        //             Cell::Empty => '.',
        //             Cell::Wall => '#',
        //         }
        //     }).collect());
        // }

        let n_actions = self.actions.len();
        for i in 0..n_actions {
            // let mut buffer = String::new();
            // io::stdin().read_line(&mut buffer);

            //match dbg!(self.actions[i]) {
            match self.actions[i] {
                Action::Move(steps) => self.move_walker(steps),
                Action::Rotate90CW => self.walker.dir = self.walker.dir.rotCW90(),
                Action::Rotate90CCW => self.walker.dir = self.walker.dir.rotCCW90(),
            }

            // let ch = match self.walker.dir {
            //     Vec2{x: 1, y: 0} => '>',
            //     Vec2{x: 0, y: 1} => 'v',
            //     Vec2{x: -1, y: 0} => '<',
            //     Vec2{x: 0, y: -1} => '^',
            //     _ => panic!("Unexpected dir: {:?}", self.walker.dir)
            // };

            // canvas[self.walker.pos.y as usize][self.walker.pos.x as usize] = ch;
            // for r in &canvas {
            //     for c in r {
            //         print!("{}", *c);
            //     }
            //     println!("");
            // }
            // println!("");
        }
    }

    fn move_walker(&mut self, steps: i32) {
        for _ in 0..steps {
            let mut nxt_pos = self.walker.pos;
            loop {
                nxt_pos = self.wrap(nxt_pos + self.walker.dir);
                match self.cell_at_pos(nxt_pos) {
                    Cell::Out => continue,
                    Cell::Empty => break,
                    Cell::Wall => return,
                }
            }
            // dbg!(nxt_pos);
            self.walker.pos = nxt_pos;
        }
    }

    fn cell_at_pos(&self, pos: Vec2) -> Cell {
        match self.field.iter().nth(pos.y as usize) {
            Some(row) => match row.iter().nth(pos.x as usize) {
                Some(cell) => *cell,
                None => Cell::Out,
            },
            None => Cell::Out,
        }
    }

    fn wrap(&self, mut pos: Vec2) -> Vec2 {
        if pos.y < 0 {
            pos.y = (self.field.len() as i32) - 1;
        }
        if (pos.y as usize) >= self.field.len() {
            pos.y = 0;
        }

        let row = self.field.iter().nth(pos.y as usize).unwrap();

        if pos.x < 0 {
            pos.x = (row.len() as i32) - 1;
        }
        if (pos.x as usize) >= self.field_width {
            pos.x = 0;
        }

        pos
    }
}

#[derive(Debug)]
struct Walker {
    pos: Vec2,
    dir: Vec2,
}

impl Walker {
    fn new(pos: Vec2) -> Self {
        // By default walker is looking towards the east
        Self {
            pos,
            dir: Vec2 { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn rotCCW90(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    fn rotCW90(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
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

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(i32),
    Rotate90CW,
    Rotate90CCW,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Out,
    Empty,
    Wall,
}

impl Cell {
    fn from_char(ch: char) -> Self {
        match ch {
            ' ' => Cell::Out,
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            _ => panic!("Invalid char to form cell: '{}'", ch),
        }
    }
}
