use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .expect("File not found");
    
    let mut max: i64 = 0;
    let mut last = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            if last > max {
                max = last;
            }
            last = 0;
            continue;
        }
        last += line.parse::<i64>().expect("Integer value expected");
    }
    if last > max {
        max = last;
    }

    println!("Answer: {}", max);
}
