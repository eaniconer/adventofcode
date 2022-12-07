use std::{cell::RefCell, env, fs, rc::Rc};

enum Command<'a> {
    Ls,
    Cd(&'a str),
}

fn parse_command(line: &str) -> Command {
    let mut splitter = line.split(' ');
    splitter.next(); // skip $

    match splitter.next().unwrap() {
        "cd" => Command::Cd(splitter.next().unwrap()),
        "ls" => Command::Ls,
        _ => unreachable!(),
    }
}

enum FsObj<'a> {
    File(&'a str, u32),
    Dir(&'a str),
}

fn parse_fs_obj(line: &str) -> FsObj {
    let mut splitter = line.trim().split(' ');
    let first = splitter.next().unwrap();
    if first == "dir" {
        FsObj::Dir(splitter.next().unwrap())
    } else {
        FsObj::File(splitter.next().unwrap(), first.parse::<u32>().unwrap())
    }
}

struct Node<'a> {
    fs_obj: FsObj<'a>,
    children: Vec<Rc<RefCell<Node<'a>>>>,
    parent: Option<Rc<RefCell<Node<'a>>>>,
}

impl<'a> Node<'a> {
    fn new_dir(name: &'a str, parent: Option<Rc<RefCell<Node<'a>>>>) -> Rc<RefCell<Node<'a>>> {
        let node = Node {
            fs_obj: FsObj::Dir(name),
            children: Vec::new(),
            parent,
        };
        Rc::new(RefCell::new(node))
    }

    fn new_file(
        name: &'a str,
        size: u32,
        parent: Option<Rc<RefCell<Node<'a>>>>,
    ) -> Rc<RefCell<Node<'a>>> {
        let node = Node {
            fs_obj: FsObj::File(name, size),
            children: Vec::new(),
            parent,
        };
        Rc::new(RefCell::new(node))
    }

    fn add_child(&mut self, node: Rc<RefCell<Node<'a>>>) {
        self.children.push(node);
    }

    fn task_1(&self) -> (u32 /* subtree size */, u32 /* task1 answer*/) {
        if let FsObj::File(_, size) = self.fs_obj {
            return (size, 0);
        }

        let mut size: u32 = 0;
        let mut answer: u32 = 0;
        for child in &self.children {
            let (s, a) = child.borrow_mut().task_1();
            size += s;
            answer += a;
        }

        if size <= 100000 {
            answer += size;
        }

        return (size, answer);
    }

    fn total_size(&self) -> u32 {
        let (size, _) = self.task_1();
        return size;
    }

    fn task_2_impl(&self, required_to_free: u32) -> (u32 /* size */, u32 /* answer */) {
        if let FsObj::File(_, size) = self.fs_obj {
            return (size, 0);
        }

        let mut size: u32 = 0;
        let mut answer: u32 = 0;
        for child in &self.children {
            let (s, a) = child.borrow_mut().task_2_impl(required_to_free);
            size += s;

            if a >= required_to_free {
                if answer == 0 || a < answer {
                    answer = a;
                }
            }
        }

        if size >= required_to_free {
            if answer == 0 || size < answer {
                answer = size;
            }
        }

        return (size, answer);
    }

    fn task_2(&self, required_to_free: u32) -> u32 {
        let (_, a) = self.task_2_impl(required_to_free);
        return a;
    }
}

fn build_tree(content: &str) -> Rc<RefCell<Node>> {
    let root = Node::new_dir("/", None);
    let mut cur = root.clone();

    for line in content.split('\n') {
        let line = line.trim();
        if line.starts_with('$') {
            let command = parse_command(line);
            if let Command::Cd(dir) = command {
                match dir {
                    "/" => cur = root.clone(),
                    ".." => {
                        let cur_clone = cur;
                        cur = cur_clone.borrow_mut().parent.as_ref().unwrap().clone();
                    }
                    _ => {
                        let mut found = false;
                        let cur_clone = cur.clone();
                        for child in &cur_clone.borrow_mut().children {
                            if let FsObj::Dir(name) = child.borrow_mut().fs_obj {
                                if name == dir {
                                    cur = child.clone();
                                    found = true;
                                    break;
                                }
                            }
                        }
                        if !found {
                            let child = Node::new_dir(dir, Some(cur.clone()));
                            cur.borrow_mut().add_child(child.clone());
                            cur = child;
                        }
                    }
                }
            }
            continue;
        }

        match parse_fs_obj(line) {
            FsObj::File(name, size) => {
                let child = Node::new_file(name, size, Some(cur.clone()));
                cur.borrow_mut().add_child(child);
            }
            FsObj::Dir(name) => {
                let child = Node::new_dir(name, Some(cur.clone()));
                cur.borrow_mut().add_child(child);
            }
        }
    }
    return root;
}

fn task_1(content: &str) -> u32 {
    let root = build_tree(content);
    let (_, answer) = root.borrow_mut().task_1();
    return answer;
}

fn task_2(content: &str) -> u32 {
    let root = build_tree(content);
    let used_space = root.borrow_mut().total_size();
    let unused_space = 70000000 - used_space;
    let required_to_free = 30000000 - unused_space;
    return root.borrow_mut().task_2(required_to_free);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("File not found");

    println!("Day7. Task1: {}", task_1(&content));
    println!("Day7. Task2: {}", task_2(&content));
}
