#![allow(dead_code)]

use std::collections::VecDeque;
use std::collections::HashMap;

// Solution could be improved by chosing a different data structure
// for `window` which could combine the functionality of the HashMap
// and VecDeque. We basically need a way to count the number of characters,
// but also push/pop on both sides of the window.

fn uniq(window: &VecDeque<char>, window_len: usize) -> bool {
    let mut counts: HashMap<char, usize> = HashMap::with_capacity(window_len);
    for c in window.iter() {
        counts.entry(*c)
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }
    println!("counts = {:?}", counts);

    counts.keys().len() == 4
}

fn find_uniq_chars(input: &String, len: usize) -> Option<usize> {
    let mut window: VecDeque<char> = VecDeque::with_capacity(len);
    for (i, c) in input.chars().enumerate() {
        if window.len() == len {
            window.pop_front();
        }
        window.push_back(c);
        if uniq(&window, len) {
            return Some(i + 1);
        }
        println!("i={:?}, window={:?}", i, window);
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::find_uniq_chars;

    #[test]
    fn test() {
        let tests = vec![
            (String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5),
            (String::from("nppdvjthqldpwncqszvftbrmjlhg"), 6),
            (String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10),
            (String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11),
        ];

        for (input, solution) in tests.into_iter() {
            println!("input = {:?}", input);
            let result = find_uniq_chars(&input, 4)
                .expect("Input had uniq sequence, but got None result");

            println!("input={:?}, result={:?}, solution={:?}",
                input, result, solution);

            assert_eq!(result, solution);
        }
    }
}
