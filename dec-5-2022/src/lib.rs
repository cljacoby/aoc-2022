use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    stacks: Vec<String>,
    moves: Vec<(usize, usize, usize)>,
}

// Part 1.
// Move crates from stacks one at a tine. This means crates moving from the `src`
// stack to the `dest` stack will have their ordering reversed.
fn crate_mover_9000(mut input: Input) -> String {
    for mv in input.moves.iter() {
        let (count, mut src, mut dest) = *mv;
        // Input move commands label stacks with 1-based numbering. So subtract 1 to get Vec index.
        src -= 1;
        dest -= 1;

        for i in 0..count {
            let c = input.stacks[src].pop().unwrap();
            input.stacks[dest].push(c);
        }
    }
    
    let mut tops = String::with_capacity(input.stacks.len());
    for stack in input.stacks.iter_mut() {
        tops.push(stack.pop().unwrap());
    }

    tops
}

// Part 2.
// Move crates from stacks in aggregate. This means crates moving from the `src`
// stack to the `dest` stack will have their ordering preserved.
fn crate_mover_9001(mut input: Input) -> String {
    for mv in input.moves.iter() {
        let (count, mut src, mut dest) = *mv;
        // Input move commands label stacks with 1-based numbering. So subtract 1 to get Vec index.
        src -= 1;
        dest -= 1;

        let mut tmp = String::with_capacity(count);
        for i in 0..count {
            let c = input.stacks[src].pop().unwrap();
            tmp.push(c);
        }

        while let Some(c) = tmp.pop() {
            input.stacks[dest].push(c);   
        }
    }
    
    let mut tops = String::with_capacity(input.stacks.len());
    for stack in input.stacks.iter_mut() {
        tops.push(stack.pop().unwrap());
    }

    tops   
}

#[cfg(test)]
mod tests {

    use crate::*;

    fn get_input() -> Input {
        let s = std::fs::read_to_string("input.json")
            .expect("Failed to read input json");
        let input: Input = serde_json::from_str(&s)
            .expect("Failed to deserialize input json");
        
        input
    }

    #[test]
    fn test_part_1() {
        let input = get_input();
        let tops = crate_mover_9000(input);
        assert_eq!(&tops, "RTGWZTHLD");
    }

    #[test]
    fn test_part_2() {
        let input = get_input();
        let tops = crate_mover_9001(input);
        assert_eq!(&tops, "STHGRZZFR");
    }
}
