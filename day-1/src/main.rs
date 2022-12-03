use std::fs;

fn main() {
    let binding = fs::read_to_string("input").expect("ruh roh!");

    let mut cal = 0;
    let mut totals = Vec::new();
    for line in binding.lines() {
        match line {
            "" => {
                totals.push(cal);
                cal = 0;
            }
            _ => {
                cal += line.parse::<i32>().unwrap();
            }
        }
    }
    let max = totals.iter().max().unwrap();
    let lucky_elf = totals.iter().position(|&r| r == *max).unwrap();
    // day 1
    println!(
        "part 1: elf {} is holding the most calories: {}",
        lucky_elf,
        max.abs()
    );

    // day 2
    totals.sort();
    let day_two_max: &i32 = &totals[totals.len() - 3..].iter().sum();
    println!("part 2: top 3 elves are carrying {} calories", day_two_max);
}
