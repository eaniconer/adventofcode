use std::{env, fs};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Won,
    Draw,
    Lost,
}

fn play_round(rival_choice: Choice, your_choice: Choice) -> Outcome {
    return match (rival_choice, your_choice) {
        (Choice::Rock, Choice::Paper) => Outcome::Won,
        (Choice::Rock, Choice::Scissors) => Outcome::Lost,
        (Choice::Paper, Choice::Rock) => Outcome::Lost,
        (Choice::Paper, Choice::Scissors) => Outcome::Won,
        (Choice::Scissors, Choice::Rock) => Outcome::Won,
        (Choice::Scissors, Choice::Paper) => Outcome::Lost,
        _ => Outcome::Draw,
    }
}

fn decode_rival_choice(choice: &str) -> Choice {
    return match choice {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Unexpected rival choice: should be 'A', 'B' or 'C', but given {}", choice)
    };
}

fn decode_your_choice(choice: &str) -> Choice {
    return match choice {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => panic!("Unexpected your choice: should be 'X', 'Y' or 'Z', but given {}", choice)
    };
}

fn choice_score(choice: Choice) -> u32 {
    return match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn outcome_score(outcome: Outcome) -> u32 {
    return match outcome {
        Outcome::Won => 6,
        Outcome::Draw => 3,
        Outcome::Lost => 0,
    }
}

fn task_1(content: &str) {
    let mut scores: u32 = 0;
    for line in content.split('\n') {
        let mut splitter = line.trim().split(' ');
        let rival_choice = splitter.next()
            .unwrap_or_else(|| panic!("Rival choice expected. Line: {}", line));
        let your_choice = splitter.next()
            .unwrap_or_else(|| panic!("Your choice expected. Line: {}", line));

        let rival_choice = decode_rival_choice(rival_choice);
        let your_choice = decode_your_choice(your_choice);
        
        let outcome = play_round(rival_choice, your_choice);
        scores += choice_score(your_choice) + outcome_score(outcome);
    }

    println!("Day2. Task1: {}", scores)
}

fn decode_outcome(choice: &str) -> Outcome {
    return match choice {
        "X" => Outcome::Lost,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Won,
        _ => panic!("Unexpected outcome: should be 'X', 'Y' or 'Z', but given {}", choice)
    };
}

fn your_choice_by_outcome(rival_choice: Choice, outcome: Outcome) -> Choice {
    return match (rival_choice, outcome) {
        (Choice::Rock, Outcome::Won) => Choice::Paper,
        (Choice::Rock, Outcome::Lost) => Choice::Scissors,
        (Choice::Paper, Outcome::Won) => Choice::Scissors,
        (Choice::Paper, Outcome::Lost) => Choice::Rock,
        (Choice::Scissors, Outcome::Won) => Choice::Rock,
        (Choice::Scissors, Outcome::Lost) => Choice::Paper,
        (x, Outcome::Draw) => x,
    }
}

fn task_2(content: &str) {
    let mut scores: u32 = 0;
    for line in content.split('\n') {
        let mut splitter = line.trim().split(' ');
        let rival_choice = splitter.next()
            .unwrap_or_else(|| panic!("Rival choice expected. Line: {}", line));
        let outcome = splitter.next()
            .unwrap_or_else(|| panic!("Outcome expected. Line: {}", line));

        let rival_choice = decode_rival_choice(rival_choice);
        let outcome = decode_outcome(outcome);
        
        let your_choice = your_choice_by_outcome(rival_choice, outcome);
        scores += choice_score(your_choice) + outcome_score(outcome);
    }

    println!("Day2. Task2: {}", scores)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .expect("File not found");

    task_1(&content);
    task_2(&content);
}
