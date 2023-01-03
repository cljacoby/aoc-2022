use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

type Pair = (usize, usize);

pub fn parse_input<P: AsRef<Path>>(path: P) -> Vec<(Pair, Pair)> {
    let mut pairs = Vec::new();
    let f = File::open(path).expect("Failed to open input file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let split: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
        assert_eq!(split.len(), 2, "Line did not contain two pairs");
        let p0 = parse_pair(&split[0]);
        let p1 = parse_pair(&split[1]);
        pairs.push((p0, p1));
    }

    pairs
}

pub fn parse_pair(s: &String) -> Pair {
    let nums: Vec<usize> = s.split('-').map(|s| s.parse::<usize>().unwrap()).collect();
    assert_eq!(nums.len(), 2, "Pair did not contain two numbers");

    (nums[0], nums[1])
}

pub fn cmp_full_overlap(p0: Pair, p1: Pair) -> bool {
    (p0.0 <= p1.0 && p0.1 >= p1.1)
        || (p1.0 <= p0.0 && p1.1 >= p0.1)
}

pub fn cmp_partial_overlap(p0: Pair, p1: Pair) -> bool {
    (p0.1 >= p1.0 && p0.0 <= p1.1)
        || (p1.1 >= p0.0 && p1.0 <= p0.1)
}

pub fn process_pairs<P: AsRef<Path>>(path: P, cmp: fn(Pair, Pair) -> bool) -> usize {
    let pairs = parse_input(path);
    let mut count = 0;
    for (p0, p1) in pairs {
        if cmp(p0, p1) {
            // println!("p0={:?}, p1={:?}", p0, p1);
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{process_pairs, cmp_full_overlap, cmp_partial_overlap};

    #[test]
    fn test_example() {
        let path = "example.txt";
        let count = process_pairs(path, cmp_full_overlap);
        assert_eq!(count, 2);
        let count = process_pairs(path, cmp_partial_overlap);
        assert_eq!(count, 4);
    }
    
    #[test]
    fn test_part_1() {
        let path = "input.txt";
        let count = process_pairs(path, cmp_full_overlap);
        assert_eq!(count, 507);
    }

    #[test]
    fn test_part_2() {
        let path = "input.txt";
        let count = process_pairs(path, cmp_partial_overlap);
        assert_eq!(count, 897);
    }
}
