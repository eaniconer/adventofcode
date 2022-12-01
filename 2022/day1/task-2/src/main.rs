use std::{fs, collections::BinaryHeap, cmp::Reverse, env};



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .expect("File not found");
    
    const N_TOP: usize = 3;

    let mut minheap = BinaryHeap::new();
    let mut add_to_heap = |i| {
        minheap.push(Reverse(i));
        while minheap.len() > N_TOP {
            minheap.pop();
        }
    };

    let mut last = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            add_to_heap(last);
            last = 0;
            continue;
        }
        last += line.parse::<i64>().expect("Integer value expected");
    }
    add_to_heap(last);

    let result: i64 = minheap.iter()
        .map(|Reverse(x)| x)
        .sum();
    

    println!("Answer: {}", result);
}