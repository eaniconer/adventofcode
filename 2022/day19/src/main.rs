use std::{
    collections::HashSet,
    env, fs,
    ops::{AddAssign, SubAssign},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    // println!("Day19. Task1: {}", task_1(&content));
    println!("Day19. Task2: {}", task_2(&content));
}

fn task_1(content: &str) -> i32 {
    let mut result: i32 = 0;

    for blueprint in content.split("\n").map(parse_blueprint) {
        let mut solver = TaskSolver1::new(blueprint);
        solver = solver.solve(24);
        result += solver.blueprint.n * solver.balance.geode;

        println!("BP {} done: {}", solver.blueprint.n, solver.balance.geode);
    }

    result
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,

    n_ore_robots: i32,
    n_clay_robots: i32,
    n_obsidian_robots: i32,
    n_geode_robots: i32,
}

impl State {
    fn default() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            n_ore_robots: 1,
            n_clay_robots: 0,
            n_obsidian_robots: 0,
            n_geode_robots: 0,
        }
    }
}

fn task_2(content: &str) -> i32 {
    let mut result: i32 = 1;

    for (idx, blueprint) in content.split("\n").map(parse_blueprint).enumerate() {
        if idx == 3 {
            break;
        }

        let mut states = HashSet::new();
        states.insert(State::default());

        for t in 1..=32 {
            let mut new_states: HashSet<State> = HashSet::new();
            for state in &states {
                if t == 23 {
                    if state.n_geode_robots < 1 {
                        continue;
                    }
                }

                let mut st = *state;
                st.ore += state.n_ore_robots;
                st.clay += state.n_clay_robots;
                st.obsidian += state.n_obsidian_robots;
                st.geode += state.n_geode_robots;

                let can_create_ore_robot = blueprint.ore_robot.ore <= state.ore;
                if can_create_ore_robot {
                    new_states.insert(State {
                        n_ore_robots: st.n_ore_robots + 1,
                        ore: st.ore - blueprint.ore_robot.ore,
                        ..st
                    });
                }

                let can_create_clay_robot = blueprint.clay_robot.ore <= state.ore;
                if can_create_clay_robot {
                    new_states.insert(State {
                        n_clay_robots: st.n_clay_robots + 1,
                        ore: st.ore - blueprint.clay_robot.ore,
                        ..st
                    });
                }

                let can_create_obsidian_robot = blueprint.obsidian_robot.ore <= state.ore
                    && blueprint.obsidian_robot.clay <= state.clay;

                if can_create_obsidian_robot {
                    new_states.insert(State {
                        n_obsidian_robots: st.n_obsidian_robots + 1,
                        ore: st.ore - blueprint.obsidian_robot.ore,
                        clay: st.clay - blueprint.obsidian_robot.clay,
                        ..st
                    });
                }

                let can_create_geode_robot = blueprint.geode_robot.ore <= state.ore
                    && blueprint.geode_robot.obsidian <= state.obsidian;
                if can_create_geode_robot {
                    new_states.insert(State {
                        n_geode_robots: st.n_geode_robots + 1,
                        ore: st.ore - blueprint.geode_robot.ore,
                        obsidian: st.obsidian - blueprint.geode_robot.obsidian,
                        ..st
                    });
                }

                new_states.insert(st);
            }

            if t == 27 {
                let v = *new_states
                    .iter()
                    .max_by_key(|s| s.geode + 5 * s.n_geode_robots)
                    .unwrap();
                let expected = v.geode + 5 * v.n_geode_robots;
                new_states.retain(|s| s.geode + 5 * s.n_geode_robots + 13 >= expected);
            }
            println!("On time {} we have {} states", t, new_states.len());

            states = new_states;
        }

        let v = states.iter().max_by_key(|s| s.geode).unwrap();
        println!("BP {} done: {}", blueprint.n, v.geode);
        result *= v.geode;
    }

    result
}

struct TaskSolver1 {
    blueprint: Blueprint,
    balance: ResourcePack, // best balance to be true
    robots: Robots,

    cur_balance: ResourcePack,
}

impl TaskSolver1 {
    fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            balance: ResourcePack::default(),
            robots: Robots::default(),
            cur_balance: ResourcePack::default(),
        }
    }

    fn solve(mut self, n_minutes: i32) -> Self {
        self.balance = ResourcePack::default();
        self.cur_balance = ResourcePack::default();
        self.step(n_minutes)
    }

    fn step(mut self, minutes_left: i32) -> Self {
        if minutes_left == 0 {
            if self.cur_balance.geode > self.balance.geode {
                self.balance = self.cur_balance;
            }
            return self;
        }

        self = self._try_step_with_create_robot(minutes_left, ResType::Geode);
        self = self._try_step_with_create_robot(minutes_left, ResType::Obsidian);
        self = self._try_step_with_create_robot(minutes_left, ResType::Ore);
        self = self._try_step_with_create_robot(minutes_left, ResType::Clay);

        let new_res = self.robots.produce();
        self.cur_balance += new_res;
        self = self.step(minutes_left - 1);
        self.cur_balance -= new_res;

        self
    }

    fn _try_step_with_create_robot(mut self, minutes_left: i32, robot_type: ResType) -> Self {
        let new_res = self.robots.produce();
        let price = self._robot_price(robot_type);

        // println!("{} Try robot {:?}", str::repeat(" |", minutes_left as usize), robot_type);
        if self._has_resources(price) {
            self.cur_balance -= price;
            self.cur_balance += new_res;
            self._change_robot(robot_type, 1);

            self = self.step(minutes_left - 1);

            self._change_robot(robot_type, -1);
            self.cur_balance += price;
            self.cur_balance -= new_res;
        } else {
            // println!("{} >Can't create robot with price {:?}", str::repeat(" |", minutes_left as usize), price);
        }
        self
    }

    fn _robot_price(&self, robot_type: ResType) -> ResourcePack {
        match robot_type {
            ResType::Ore => self.blueprint.ore_robot,
            ResType::Clay => self.blueprint.clay_robot,
            ResType::Obsidian => self.blueprint.obsidian_robot,
            ResType::Geode => self.blueprint.geode_robot,
        }
    }

    fn _change_robot(&mut self, robot_type: ResType, n: i32) {
        match robot_type {
            ResType::Ore => self.robots.n_ore_robots += n,
            ResType::Clay => self.robots.n_clay_robots += n,
            ResType::Obsidian => self.robots.n_obsidian_robots += n,
            ResType::Geode => self.robots.n_geode_robots += n,
        }
    }

    fn _has_resources(&self, price: ResourcePack) -> bool {
        price.ore <= self.cur_balance.ore
            && price.clay <= self.cur_balance.clay
            && price.obsidian <= self.cur_balance.obsidian
            && price.geode <= self.cur_balance.geode
    }

    fn _robot_can_be_created(&self, robot_type: ResType) -> bool {
        let price = self._robot_price(robot_type);
        self._has_resources(price)
    }
}

#[derive(Debug, Clone, Copy)]
enum ResType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Blueprint {
    n: i32,

    ore_robot: ResourcePack,
    clay_robot: ResourcePack,
    obsidian_robot: ResourcePack,
    geode_robot: ResourcePack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResourcePack {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl ResourcePack {
    fn default() -> Self {
        ResourcePack {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn has_resources(&self, price: ResourcePack) -> bool {
        price.ore <= self.ore
            && price.clay <= self.clay
            && price.obsidian <= self.obsidian
            && price.geode <= self.geode
    }
}

impl SubAssign for ResourcePack {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore.sub_assign(rhs.ore);
        self.clay.sub_assign(rhs.clay);
        self.obsidian.sub_assign(rhs.obsidian);
        self.geode.sub_assign(rhs.geode);
    }
}

impl AddAssign for ResourcePack {
    fn add_assign(&mut self, rhs: Self) {
        self.ore.add_assign(rhs.ore);
        self.clay.add_assign(rhs.clay);
        self.obsidian.add_assign(rhs.obsidian);
        self.geode.add_assign(rhs.geode);
    }
}

struct Robots {
    n_ore_robots: i32,
    n_clay_robots: i32,
    n_obsidian_robots: i32,
    n_geode_robots: i32,
}

impl Robots {
    fn default() -> Self {
        Self {
            n_ore_robots: 1,
            n_clay_robots: 0,
            n_obsidian_robots: 0,
            n_geode_robots: 0,
        }
    }

    fn produce(&self) -> ResourcePack {
        ResourcePack {
            ore: self.n_ore_robots,
            clay: self.n_clay_robots,
            obsidian: self.n_obsidian_robots,
            geode: self.n_geode_robots,
        }
    }
}

fn parse_blueprint(line: &str) -> Blueprint {
    let mut splitter = line.trim().split(": ");

    let n: i32 = splitter
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let mut splitter = splitter.next().unwrap().split(". ");

    Blueprint {
        n,
        ore_robot: parse_respack(splitter.next().unwrap()),
        clay_robot: parse_respack(splitter.next().unwrap()),
        obsidian_robot: parse_respack(splitter.next().unwrap()),
        geode_robot: parse_respack(splitter.next().unwrap()),
    }
}

fn parse_respack(line: &str) -> ResourcePack {
    let mut splitter = line.trim().split(" costs ");
    splitter.next();

    let mut respack = ResourcePack::default();

    for res in splitter.next().unwrap().split(" and ") {
        let res = if res.ends_with(".") {
            res.get(..res.len() - 1).unwrap()
        } else {
            res
        };

        let mut res_splitter = res.split(' ');
        let count: i32 = res_splitter.next().unwrap().parse().unwrap();
        match res_splitter.next().unwrap() {
            "ore" => respack.ore += count,
            "clay" => respack.clay += count,
            "obsidian" => respack.obsidian += count,
            "geode" => respack.geode += count,
            other => panic!("Unexpected type of resource: '{}' in '{}'", other, res),
        }
    }

    respack
}
