use std::{env, fs, collections::{HashSet}};

fn rucksack_checker(rucksack: &str) {
    if rucksack.len() % 2 == 1 {
        panic!("Error: invalid length. Rucksack: {}", rucksack);
    }
    if rucksack.is_empty() {
        panic!("Error: empty rucksack");
    }
}

fn score(ch: char) -> i32 {
    if 'a' <= ch && ch <= 'z' {
        return (ch as i32) - ('a' as i32) + 1;
    }
    if 'A' <= ch && ch <= 'Z' {
        return (ch as i32) - ('A' as i32) + 27;
    }
    panic!("Unexpected char: '{}'({})", ch, ch as i32)
}

fn rucksack_prioirty((lhs, rhs): (&str, &str)) -> i32 {
    let hs: HashSet<char> = lhs.chars().collect();
    let found = rhs.chars()
        .find(|ch| hs.contains(ch))
        .expect("Warranty violated: lhs and rhs don't containt similar items");
    return score(found);
}

fn task_1(content: &str) -> i32 {
    return content.split('\n')
        .map(|s| s.trim())
        .inspect(|s| rucksack_checker(s))
        .map(|x| x.split_at(x.len()/2))
        .map(rucksack_prioirty)
        .sum();
}

fn group_priority(group: &Vec<&str>) -> i32 {
    let mut hashsets = group.iter()
        .map(|x| x.chars().collect::<HashSet<char>>());

    let mut hs = hashsets.next().unwrap();
    for hashset in hashsets {
        hs = hs.intersection(&hashset).copied().collect();
    }

    if hs.len() != 1 {
        panic!("Intersection of rucksacks should consist of 1 item")
    }

    return score(*hs.iter().next().unwrap());
}

fn task_2(content: &str) -> i32 {
    let mut lines = content.split('\n')
        .map(|s| s.trim())
        .inspect(|s| rucksack_checker(s))
        .peekable();
     
    let mut result = 0;
    while lines.peek().is_some() {
        let group: Vec<_> = lines.by_ref().take(3).collect();
        result += group_priority(&group);
    }
    
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .expect("File not found");

    println!("Day3. Task1: {}", task_1(&content));
    println!("Day3. Task2: {}", task_2(&content));
}
