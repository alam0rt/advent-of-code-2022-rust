use std::fs;

enum Hand {
    None,
    Rock,
    Paper,
    Scissors,
}

fn value(hand: &Hand) -> i32 {
    match hand {
        Hand::None => -1,
        Hand::Rock => 0,
        Hand::Paper => 1,
        Hand::Scissors => 2,
    }
}

fn score(hand: &Hand) -> i32 {
    match hand {
        Hand::None => 0,
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn hand(hand: &str) -> Hand {
    match hand {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => Hand::None,
    }
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn outcome(us: &Hand, them: &Hand) -> Outcome {
    if value(us) == value(them) {
        Outcome::Draw
    } else if (value(us) + 1) % 3 == value(them) {
        Outcome::Lose
    } else {
        Outcome::Win
    }
}

fn to_win(hand: &Hand) -> Hand {
    match hand {
        Hand::Rock => Hand::Paper,
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
        _ => Hand::None,
    }
}

fn to_lose(hand: &Hand) -> Hand {
    match hand {
        Hand::Rock => Hand::Scissors,
        Hand::Paper => Hand::Rock,
        Hand::Scissors => Hand::Paper,
        _ => Hand::None,
    }
}

fn desired(us: &Hand) -> Outcome {
    match us {
        Hand::Rock => Outcome::Lose,
        Hand::Paper => Outcome::Draw,
        Hand::Scissors => Outcome::Win,
        _ => {
            panic!("uh oh");
        }
    }
}

fn scoring(us: &Hand, them: &Hand) -> i32 {
    match outcome(us, them) {
        Outcome::Draw => 3 + score(us),
        Outcome::Lose => score(us),
        Outcome::Win => 6 + score(us),
    }
}

fn main() {
    let binding = fs::read_to_string("input").expect("can't read input");

    let mut total_score_part_one = 0;
    let mut total_score_part_two = 0;

    for line in binding.lines() {
        let mut round = line.split_whitespace();
        let them = hand(round.next().unwrap());
        let us = hand(round.next().unwrap());

        total_score_part_one += scoring(&us, &them);
        total_score_part_two += match desired(&us) {
            Outcome::Draw => scoring(&them, &them),
            Outcome::Lose => scoring(&to_lose(&them), &them),
            Outcome::Win => scoring(&to_win(&them), &them),
        };
    }
    println!("part 1: {}", total_score_part_one);
    println!("part 2: {}", total_score_part_two);
}
