#![allow(dead_code)]

use std::convert::AsRef;
use std::path::Path;

/**
 * ASSUMPTIONS:
 *  - Length of row i=0 is length for all rows (i.e. rectangular grid)
 *  - Grid row len > 0
*/

fn is_visible(val: u32, coords: Vec<(usize, usize)>, grid: &Vec<Vec<u32>>) -> bool {
    for (x, y) in coords {
        if val <= grid[x][y] {
            return false;
        }
    }

    true
}

fn scenic_score(val: u32, coords: Vec<(usize, usize)>, grid: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;

    for (x, y) in coords {
        count += 1;
        if val <= grid[x][y] {
            return count;
        }
    }

    count
}

fn perimeter_trees(length: usize, width: usize) -> usize {
    (2 * length) + 2 * (width - 2)
}

fn visible_trees(grid: &Vec<Vec<u32>>) -> usize {
    let i_len = grid.len();
    let j_len = grid[0].len();
    let mut visible_trees = 0;
    let perim = perimeter_trees(i_len, j_len);

    for i in 1..(i_len - 1) {
        for j in 1..(j_len - 1) {
            let left: Vec<(usize, usize)> = (0..j).map(|y| (i, y)).collect();
            let right: Vec<(usize, usize)> = (j + 1..j_len).map(|y| (i, y)).collect();
            let up: Vec<(usize, usize)> = (0..i).map(|x| (x, j)).collect();
            let down: Vec<(usize, usize)> = (i + 1..i_len).map(|x| (x, j)).collect();
            if is_visible(grid[i][j], left, grid)
                || is_visible(grid[i][j], right, grid)
                || is_visible(grid[i][j], up, grid)
                || is_visible(grid[i][j], down, grid)
            {
                visible_trees += 1;
            }
        }
    }

    perim + visible_trees
}

fn max_scenic_score(grid: &Vec<Vec<u32>>) -> u32 {
    let i_len = grid.len();
    let j_len = grid[0].len();
    let mut max_scenic_score = 0;

    for i in 1..(i_len - 1) {
        for j in 1..(j_len - 1) {
            let left: Vec<(usize, usize)> = (0..j).map(|y| (i, y)).rev().collect();
            let right: Vec<(usize, usize)> = (j + 1..j_len).map(|y| (i, y)).collect();
            let up: Vec<(usize, usize)> = (0..i).map(|x| (x, j)).rev().collect();
            let down: Vec<(usize, usize)> = (i + 1..i_len).map(|x| (x, j)).collect();
            let left_score = scenic_score(grid[i][j], left.clone(), grid);
            let right_score = scenic_score(grid[i][j], right.clone(), grid);
            let up_score = scenic_score(grid[i][j], up.clone(), grid);
            let down_score = scenic_score(grid[i][j], down.clone(), grid);
            let score = left_score * right_score * up_score * down_score;
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    max_scenic_score
}

fn file_to_grid<P: AsRef<Path>>(path: P) -> Vec<Vec<u32>> {
    std::fs::read_to_string(path)
        .expect("Failed to read input file.")
        .split("\n")
        .map(|line| line.to_string())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_part_1() {
        let grid = file_to_grid("example.txt");
        let count = visible_trees(&grid);
        assert_eq!(count, 21, "Count of visible trees was incorrect");
    }

    #[test]
    fn part_1() {
        let grid = file_to_grid("input.txt");
        let count = visible_trees(&grid);
        assert_eq!(count, 1805, "Count of visible trees was incorrect");
    }

    #[test]
    fn example_part_2() {
        let grid = file_to_grid("example.txt");
        let count = max_scenic_score(&grid);
        assert_eq!(count, 8, "Max scenic score was incorrect");
    }

    #[test]
    fn part_2() {
        let grid = file_to_grid("input.txt");
        let count = max_scenic_score(&grid);
        assert_eq!(count, 444528, "Max scenic score was incorrect");
    }
}
