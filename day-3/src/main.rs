use std::fs::File;
use std::io::{self, Read};

struct Rucksack<'a>(&'a str, &'a str);

impl<'a> From<(&'a str, &'a str)> for Rucksack<'a> {
    fn from(i: (&'a str, &'a str)) -> Self {
        Self(i.0, i.1)
    }
}

fn main() {
    println!("Hello, world!");
    let input_data = match read_input("include/input") {
        Ok(input) => input,
        Err(e) => panic!("{}", e),
    };

    let _rucksacks = parse_input(&input_data);
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
        for rucksack in rucksacks {
            println!(
                "{}[{}] : {}[{}]",
                rucksack.0,
                rucksack.0.len(),
                rucksack.1,
                rucksack.1.len()
            );
            for b in rucksack.0.as_bytes() {
                if b.is_ascii_lowercase() {
                    println!("{}", b - 96);
                } else {
                    println!("{}", b - 38);
                }
            }
        }
    }
}
