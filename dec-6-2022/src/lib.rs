#![allow(dead_code)]

use std::collections::VecDeque;
use std::collections::HashMap;

// Solution could be improved by chosing a different data structure
// for `window` which could combine the functionality of the HashMap
// and VecDeque. We basically need a way to count the number of characters
// as we go, instead of recreating at each iteration, but also push/pop on
// both sides of the window.

fn uniq(window: &VecDeque<char>, window_len: usize) -> bool {
    let mut counts: HashMap<char, usize> = HashMap::with_capacity(window_len);
    for c in window.iter() {
        counts.entry(*c)
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }

    counts.keys().len() == window_len
}

fn find_uniq_chars(input: &String, window_len: usize) -> Option<usize> {
    let mut window: VecDeque<char> = VecDeque::with_capacity(window_len);
    for (i, c) in input.chars().enumerate() {
        if window.len() == window_len {
            window.pop_front();
        }
        window.push_back(c);
        if uniq(&window, window_len) {
            return Some(i + 1);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {

    use crate::*;

    fn test(window_len: usize, tests: Vec<(String, usize)>) {
        for (input, solution) in tests.into_iter() {
            let result = find_uniq_chars(&input, window_len)
                .expect("Input had uniq sequence, but got None result");

            println!("input={:?}, result={:?}, solution={:?}",
                input, result, solution);

            assert_eq!(result, solution);
        }
    }

    #[test]
    fn test_part_1() {
        let cases = vec![
            (String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5),
            (String::from("nppdvjthqldpwncqszvftbrmjlhg"), 6),
            (String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10),
            (String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11),
            (std::fs::read_to_string("input.txt").unwrap(), 1175),
        ];
        
        let window_len = 4;
        test(window_len, cases);
    }
    
    #[test]
    fn test_part_2() {
        let cases = vec![
            (String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19),
            (String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23),
            (String::from("nppdvjthqldpwncqszvftbrmjlhg"), 23),
            (String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29),
            (String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26),
            (std::fs::read_to_string("input.txt").unwrap(), 3217),
        ];
        
        let window_len = 14;
        test(window_len, cases);
    }
}
