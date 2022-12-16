use std::{
    collections::{btree_map::Values, HashMap, HashSet},
    env, fs,
    hash::Hash,
    io,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day16. Task1: {}", task(1, &content));
    println!("Day16. Task2: {}", task(2, &content));
}

fn task(task_n: i32, content: &str) -> u32 {
    let valves: Vec<Valve> = content.split("\n").map(parse_valve).collect();

    let mut v2i: HashMap<&str, usize> = HashMap::new(); // valve2index;
    let mut i2v: HashMap<usize, &Valve> = HashMap::new();
    for (idx, valve) in valves.iter().enumerate() {
        v2i.insert(&valve.name, idx);
        i2v.insert(idx, &valve);
    }

    let sz = valves.len();
    let mut ws: Vec<Vec<u32>> = vec![vec![u32::MAX; sz]; sz]; // edge weights matrix
    for valve in &valves {
        let i = v2i.get(&valve.name[..]).unwrap();
        for friend in &valve.friends {
            let j = v2i.get(&friend[..]).unwrap();
            ws[*i][*j] = 1;
            ws[*j][*i] = 1;
        }
    }

    for k in 0..sz {
        for i in 0..sz {
            for j in 0..sz {
                let v = ws[i][k].saturating_add(ws[k][j]);
                ws[i][j] = ws[i][j].min(v);
            }
        }
    }

    let mut valve_assignee = vec![ValveAssignee::Man; sz];
    let mut no_assignee_counter = 0;
    let mut rate_sum = 0;
    for i in 0..sz {
        let valve = *i2v.get(&i).unwrap();
        if valve.rate == 0 && valve.name != "AA" {
            for j in 0..sz {
                ws[i][j] = 0;
                ws[j][i] = 0;
            }
            valve_assignee[i] = ValveAssignee::No;
            no_assignee_counter += 1;
        }
        rate_sum += valve.rate;
    }

    let significant_valve_counter = sz - no_assignee_counter - 1; // -1 stands for 'AA'
                                                                  // println!("Valves to consider count: {}", significant_valve_counter);

    // for i in 0..sz {
    //     println!("{} - {}", i, i2v.get(&i).unwrap().name);
    // }

    // for row in &ws {
    //     for item in row {
    //         if *item == u32::MAX {
    //             print!("x ");
    //         } else {
    //             print!("{} ", item);
    //         }
    //     }
    //     println!("");
    // }

    if task_n == 1 {
        let mut walker = Walker1::new(&i2v, &ws);
        walker = walker.bfs(*v2i.get("AA").unwrap(), 0, rate_sum);
        return walker.best_solution;
    }

    if task_n == 2 {
        let mut ans = 0;

        let n = 1 << significant_valve_counter;
        for i in 1..n {
            // if i % 100 == 0 {
            //     println!("{}/{}", i, n);
            // }

            let mut idx: usize = 0;

            let value_assignee: Vec<ValveAssignee> = valve_assignee
                .iter()
                .map(|it| match it {
                    ValveAssignee::No => *it,
                    _ => {
                        if (i & (1 << idx)) > 0 {
                            idx += 1;
                            ValveAssignee::Man
                        } else {
                            idx += 1;
                            ValveAssignee::Elephant
                        }
                    }
                })
                .collect();

            let mut man = Walker1::new2(&i2v, &ws, value_assignee.clone(), ValveAssignee::Man, 26);
            man = man.bfs(*v2i.get("AA").unwrap(), 0, rate_sum);

            let mut elephant =
                Walker1::new2(&i2v, &ws, value_assignee, ValveAssignee::Elephant, 26);
            elephant = elephant.bfs(*v2i.get("AA").unwrap(), 0, rate_sum);

            let sm = man.best_solution + elephant.best_solution;
            ans = ans.max(sm);
        }

        return ans;
    }

    unreachable!()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ValveAssignee {
    No,
    Man,
    Elephant,
}

struct Walker1<'a> {
    i2v: &'a HashMap<usize, &'a Valve>,
    ws: &'a Vec<Vec<u32>>,

    visited: Vec<bool>,
    time: u32,
    best_solution: u32,

    whose: ValveAssignee,
    valve_assignee: Vec<ValveAssignee>,
}

impl<'a> Walker1<'a> {
    fn new(i2v: &'a HashMap<usize, &'a Valve>, ws: &'a Vec<Vec<u32>>) -> Self {
        Self {
            i2v,
            ws,
            visited: vec![false; 100],
            time: 30,
            best_solution: 0,
            whose: ValveAssignee::No,
            valve_assignee: Vec::new(),
        }
    }

    fn new2(
        i2v: &'a HashMap<usize, &'a Valve>,
        ws: &'a Vec<Vec<u32>>,
        valve_assignee: Vec<ValveAssignee>,
        whose: ValveAssignee,
        time: u32,
    ) -> Self {
        Self {
            i2v,
            ws,
            visited: vec![false; 100],
            time,
            best_solution: 0,
            whose,
            valve_assignee,
        }
    }

    fn bfs(mut self, node: usize, released: u32, rest_rate: u32) -> Self {

        self.visited[node] = true;

        if released + rest_rate * self.time < self.best_solution {
            self.visited[node] = false;
            return self;
        }

        if self.time == 0 {
            self.visited[node] = false;
            return self;
        }

        let rate = self.i2v.get(&node).unwrap().rate;
        if rate > 0 {
            self.time -= 1;
        }

        let released_here = rate * self.time;
        self.best_solution = self.best_solution.max(released + released_here);

        // println!("{} Visiting {} with rate {} and released here {}. rem.time {}", str::repeat(" ", self.time as usize),
        //     self.i2v.get(&node).unwrap().name , rate, released_here, self.time);

        for nxt in 0..self.ws.len() {
            // println!("{} Try to visit {}, host: {}", str::repeat(" ", self.time as usize),
            //     self.i2v.get(&nxt).unwrap().name, self.i2v.get(&node).unwrap().name);
            if self.whose != ValveAssignee::No {
                if self.valve_assignee[nxt] != self.whose {
                    continue;
                }
            }

            if self.visited[nxt] {
                continue;
            }
            let w = self.ws[node][nxt];
            if w == 0 {
                continue;
            }
            if w > self.time {
                continue;
            }

            self.time -= w;
            self = self.bfs(nxt, released + released_here, rest_rate - rate);
            self.time += w;
        }

        if rate > 0 {
            self.time += 1;
        }

        self.visited[node] = false;
        self
    }
}

struct Valve {
    name: String,
    rate: u32,
    friends: Vec<String>,
}

impl Valve {
    fn new(name: String, rate: u32, friends: Vec<String>) -> Self {
        Self {
            name,
            rate,
            friends,
        }
    }
}

fn parse_valve(line: &str) -> Valve {
    let mut sp = line.trim().split(" ");
    sp.next(); // Skip 'Valve'
    let name = sp.next().unwrap().to_string();
    sp.next(); // Skip 'has'
    sp.next(); // Skip 'flow'
    let rate = parse_rate(sp.next().unwrap());
    sp.next(); // Skip 'tunnel(s)'
    sp.next(); // Skip 'lead(s)
    sp.next(); // Skip 'to'
    sp.next(); // Skip 'valve(s)'
    let mut friends = Vec::new();

    while let Some(friend) = sp.next() {
        friends.push(
            if friend.ends_with(',') {
                friend.get(..friend.len() - 1).unwrap()
            } else {
                friend
            }
            .to_string(),
        )
    }

    return Valve {
        name,
        rate,
        friends,
    };
}

fn parse_rate(line: &str) -> u32 {
    let n = line.split("=").nth(1).unwrap();
    n.get(..n.len() - 1).unwrap().parse().unwrap()
}
