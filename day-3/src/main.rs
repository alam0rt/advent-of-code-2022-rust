use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct Rucksack<'a>(&'a str, &'a str);

impl<'a> From<(&'a str, &'a str)> for Rucksack<'a> {
    fn from(i: (&'a str, &'a str)) -> Self {
        Self(i.0, i.1)
    }
}

fn main() {
    println!("Hello, world!");
    let input_data: String = match read_input("include/input") {
        Ok(input) => input,
        Err(e) => panic!("{}", e),
    };

    let rucksacks = parse_input(input_data.as_str());
    let mut sum: i32 = 0;
    for rucksack in rucksacks {
        let common_items = get_unique(rucksack);
        for i in common_items {
            sum += i as i32;
        }
    }
    println!("sum is {}", sum);
}

fn to_priority(i: u8) -> Result<u8, &'static str> {
    if i.is_ascii_lowercase() {
        Ok(i - 96)
    } else if i.is_ascii_uppercase() {
        Ok(i - 38)
    } else {
        Err("not a valid item")
    }
}

fn get_unique(r: Rucksack) -> Vec<u8> {
    println!("{:?}", r);
    let mut unique_map: HashSet<u8> = HashSet::with_capacity(26 * 2); // alphabet * 2
    for item in r.0.as_bytes().iter() {
        unique_map.insert(*item);
    }

    r.1.as_bytes()
        .iter()
        .copied()
        .filter(|b| unique_map.contains(b))
        .collect::<HashSet<u8>>()
        .into_iter()
        .collect::<Vec<u8>>()
        .into_iter()
        .map(|p| to_priority(p).unwrap())
        .collect()
}

fn read_input(path: &str) -> Result<String, io::Error> {
    let input_file_result = File::open(path);

    let mut input_file = match input_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut input = String::new();

    match input_file.read_to_string(&mut input) {
        Ok(_) => Ok(input),
        Err(e) => Err(e),
    }
}

fn parse_input(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|e| Rucksack::from(e.split_at(&e.len() / 2)))
        .collect::<Vec<Rucksack>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_data = read_input("include/example");
        let input_error = input_data.as_ref().err();
        assert!(input_error.is_none());

        let rucksacks = parse_input(input_data.as_ref().unwrap().as_str());
        let mut sum = 0;
        for rucksack in rucksacks {
            let common_items = get_unique(rucksack);
            for i in common_items {
                sum += i;
            }
        }
        assert_eq!(sum, 157);
    }
}
