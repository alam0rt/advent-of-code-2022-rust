use std::collections::{HashMap, HashSet};
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
        let r = vec![rucksack.0, rucksack.1];
        let common_items = get_unique(&r[..]);
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

fn get_badges(rs: &[Rucksack]) -> Vec<u8> {
    let mut result = vec![];
    for triplet in rs.chunks(3) {
        let triplets = triplet
            .iter()
            .map(|r| format!("{}{}", r.0, r.1))
            .collect::<Vec<String>>();

        let striplets = triplets.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

        let badge = get_unique(&striplets[..]);
        println!("badges: {:?}", &badge);

        result.extend(badge);
    }
    result
}

fn get_unique(strings: &[&str]) -> Vec<u8> {
    let mut sorted: Vec<u8> = vec![];
    for r in strings {
        let mut bytes = r.as_bytes().to_vec();
        bytes.sort();
        bytes.dedup();
        sorted.extend(bytes);
    }

    println!("{:?}", &sorted);

    let mut map = HashMap::<u8, usize>::new();

    for r in sorted.iter() {
        match map.get(r) {
            None => {
                map.insert(*r, 1);
            }
            Some(v) => {
                map.insert(*r, v + 1);
            }
        }
    }

    println!("{:?}", &map);

    map.iter()
        .filter(|&(_, v)| strings.len() == *v)
        .map(|(k, _)| *k)
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
        let mut sum: i32 = 0;
        for rucksack in rucksacks {
            let r = vec![rucksack.0, rucksack.1];
            let common_items = get_unique(&r[..]);
            println!("{:?}", common_items);
            for i in common_items {
                println!("{}", i);
                match to_priority(i) {
                    Ok(v) => sum += v as i32,
                    Err(e) => panic!("{}", e),
                }
            }
        }
        println!("sum: {}", &sum);
        assert_eq!(sum, 157);
    }

    #[test]
    fn test_part_two() {
        let input_data = read_input("include/example");
        let input_error = input_data.as_ref().err();
        assert!(input_error.is_none());

        let rucksacks = parse_input(input_data.as_ref().unwrap().as_str());
        let badges = get_badges(&rucksacks);

        let mut sum = 0;
        for badge in badges {
            match to_priority(badge) {
                Ok(v) => sum += v as i32,
                Err(e) => panic!("{}", e),
            }
        }
        assert_eq!(sum, 70);
    }
}
