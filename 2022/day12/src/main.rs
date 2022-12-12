use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day12. Task1: {}", task_1(&content));
    println!("Day12. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> u32 {
    let hm = Heightmap::from(content);
    let end = hm.end();
    let start = hm.start();

    let spf = ShortPathFinder::new(hm, end);

    return spf.len_to(start).unwrap();
}

fn task_2(content: &str) -> u32 {
    let hm = Heightmap::from(content);
    let end = hm.end();
    let start = hm.start();

    let spf = ShortPathFinder::new(hm, end);

    let mut p = spf.len_to(start).unwrap();

    for (r, row) in spf.heightmap.field.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == 'a' {
                if let Some(np) = spf.len_to(Pos { r, c }) {
                    if np < p {
                        p = np;
                    }
                }
            }
        }
    }

    return p;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    r: usize,
    c: usize,
}

struct Heightmap {
    field: Vec<Vec<char>>,
}

impl Heightmap {
    fn from(content: &str) -> Self {
        Self {
            field: content.split("\n").map(|r| r.chars().collect()).collect(),
        }
    }

    fn w(&self) -> usize {
        self.field[0].len()
    }
    fn h(&self) -> usize {
        self.field.len()
    }

    fn start(&self) -> Pos {
        self._find_sym('S')
    }

    fn end(&self) -> Pos {
        self._find_sym('E')
    }

    fn _find_sym(&self, sym: char) -> Pos {
        for (r, row) in self.field.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == sym {
                    return Pos { r, c };
                }
            }
        }
        unreachable!()
    }
}

struct ShortPathFinder {
    heightmap: Heightmap,
    meta: Vec<Vec<Option<u32>>>,
}

impl ShortPathFinder {
    fn new(heightmap: Heightmap, endpoint: Pos) -> Self {
        let w = heightmap.w();
        let h = heightmap.h();
        let mut ret = Self {
            heightmap,
            meta: vec![vec![None; w]; h],
        };

        ret.meta[endpoint.r][endpoint.c] = Some(0);
        ret._precalc_path(endpoint);

        ret
    }

    fn _neighs(&self, p: Pos) -> Vec<Pos> {
        let mut neighs = Vec::new();

        if p.r > 0 {
            neighs.push(Pos { r: p.r - 1, c: p.c });
        }
        if p.c > 0 {
            neighs.push(Pos { r: p.r, c: p.c - 1 });
        }
        if p.r + 1 != self.heightmap.h() {
            neighs.push(Pos { r: p.r + 1, c: p.c });
        }
        if p.c + 1 != self.heightmap.w() {
            neighs.push(Pos { r: p.r, c: p.c + 1 });
        }

        neighs
    }

    fn _height(&self, pos: Pos) -> u32 {
        match self.heightmap.field[pos.r][pos.c] {
            'E' => ('z' as u32) + 1,
            'S' => ('a' as u32) - 1,
            ch => (ch as u32),
        }
    }

    fn _precalc_path(&mut self, endpoint: Pos) {
        let mut wave = vec![endpoint];
        let mut nxt_len: u32 = 1;

        while !wave.is_empty() {
            let mut nxt_wave = Vec::new();

            for p in wave {
                let p_h = self._height(p);
                for n in self._neighs(p) {
                    if self.meta[n.r][n.c].is_some() {
                        continue;
                    }
                    let n_h = self._height(n);
                    if p_h == n_h || p_h == n_h + 1 || n_h > p_h {
                        self.meta[n.r][n.c] = Some(nxt_len);
                        nxt_wave.push(n);
                    }
                }
            }

            nxt_len += 1;
            wave = nxt_wave;

            // println!("WAVE {}", nxt_len - 1);
            // for r in 0..self.heightmap.h() {
            //     for c in 0..self.heightmap.w() {
            //         let ch = if self.meta[r][c].is_some() {
            //             self.heightmap.field[r][c]
            //         } else {
            //             '.'
            //         };
            //         print!("{}", ch);
            //     }
            //     println!("")
            // }
        }
    }

    fn len_to(&self, p: Pos) -> Option<u32> {
        self.meta[p.r][p.c]
    }
}
