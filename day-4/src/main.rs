use std::fs::File;
use std::io::{self, Read};

fn main() {
    result();
}

fn result() {
    let input = read_input("include/input").unwrap();

    let mut full_overlap = 0;
    let mut partial_overlap = 0;

    for line in input.lines() {
        let pairs = get_pairs(line);
        if is_overlapping(&pairs[..]) {
            full_overlap += 1;
        }
        if is_partial_overlap(&pairs[..]) {
            partial_overlap += 1;
        }
    }
    println!("part one: {}", full_overlap);
    println!("part two: {}", partial_overlap);
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

fn is_partial_overlap(pairs: &[u8]) -> bool {
    match pairs {
        [x1, x2, y1, y2] => x1.le(y2) && y1.le(x2),
        _ => panic!("incorrectly sized slice provided! must be a pair of pairs"),
    }
}

fn is_overlapping(pairs: &[u8]) -> bool {
    match pairs {
        [x1, x2, y1, y2] => x1.le(y1) && x2.ge(y2) || y1.le(x1) && y2.ge(x2),
        _ => panic!("incorrectly sized slice provided! must be a pair of pairs"),
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
    fn test_is_partial_overlap() {
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
            let got = is_partial_overlap(pairs);
            assert!(got);
        }
        {
            let pairs: &[u8; 4] = &[2, 8, 3, 7];
            let got = is_partial_overlap(pairs);
            assert!(got);
        }
        {
            let pairs: &[u8; 4] = &[6, 6, 4, 6];
            let got = is_partial_overlap(pairs);
            assert!(got);
        }
        {
            let pairs: &[u8; 4] = &[2, 6, 4, 8];
            let got = is_partial_overlap(pairs);
            assert!(got);
        }
    }

    #[test]
    fn test_part_one() {
        let input = read_input("include/example").unwrap();

        let mut overlapping = 0;
        for line in input.lines() {
            let pairs = get_pairs(line);
            if is_overlapping(&pairs[..]) {
                overlapping += 1;
            }
        }
        assert_eq!(overlapping, 2);
    }

    #[test]
    fn test_part_two() {
        let input = read_input("include/example").unwrap();

        let mut overlapping = 0;
        for line in input.lines() {
            let pairs = get_pairs(line);
            if is_partial_overlap(&pairs[..]) {
                overlapping += 1;
                println!("{} (overlapping)", line);
            } else {
                println!("{}", line);
            }
        }
        assert_eq!(overlapping, 4);
    }
}
