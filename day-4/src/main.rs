use std::fs::File;
use std::io::{self, BufRead, Read};

fn main() {
    println!("Hello, world!");
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

fn get_pairs(line: &str) -> Vec<u8> {
    let r = line
        .split(',')
        .flat_map(|p| {
            p.split('-').map(|v| match str::parse::<u8>(v) {
                Ok(v) => v,
                Err(e) => panic!("error parsing {}: {}", v, e),
            })
        })
        .collect::<Vec<u8>>();

    r
}

fn is_overlapping(pairs: &[u8; 4]) -> bool {
    match pairs {
        [l1, r1, l2, r2] => l1.le(l2) && r1.ge(r2) || l2.le(l1) && r2.ge(r1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_overlapping() {
        {
            let pairs: &[u8; 4] = &[2, 3, 4, 5];
            let got = is_overlapping(pairs);
            assert!(!got);
        }
        {
            let pairs: &[u8; 4] = &[2, 4, 6, 8];
            let got = is_overlapping(pairs);
            assert!(!got);
        }
        {
            let pairs: &[u8; 4] = &[5, 7, 7, 9];
            let got = is_overlapping(pairs);
            assert!(!got);
        }
        {
            let pairs: &[u8; 4] = &[2, 8, 3, 7];
            let got = is_overlapping(pairs);
            assert!(got);
        }
        {
            let pairs: &[u8; 4] = &[6, 6, 4, 6];
            let got = is_overlapping(pairs);
            assert!(got);
        }
        {
            let pairs: &[u8; 4] = &[2, 6, 4, 8];
            let got = is_overlapping(pairs);
            assert!(!got);
        }
    }

    #[test]
    fn test_part_one() {
        let input = read_input("include/example").unwrap();
        println!("{}", input);
        for line in input.lines() {
            let pairs = get_pairs(line);
            println!("{:?}", pairs);
        }
    }
}
