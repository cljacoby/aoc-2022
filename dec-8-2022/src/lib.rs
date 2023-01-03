use std::path::Path;
use std::convert::AsRef;

/** 
 * ASSUMPTIONS:
 *  - Length of row i=0 is length for all rows (i.e. rectangular grid)
 *  - Grid row len > 0
*/

enum Direction {
    Up,
    Right,
    Down,
    Left
}


fn visible_trees(grid: Vec<Vec<u32>>) {
    let i_len = grid.len();
    let i_len = grid.len();
    let j_len = grid[0].len();

    for i in 1 .. (i_len - 1) {
        for j in 1 .. (j_len - 1) {
            println!("grid[{:?}][{:?}]={:?}", i, j,  grid[i][j]);
            // horizontal left
            let coords: Vec<(usize, usize)> = (0 .. j)
                .map(|y| (i, y))
                .collect();
            println!("horizontal_left = {:?}", coords);
        }
    }

}


fn file_to_grid<P: AsRef<Path>>(path: P) -> Vec<Vec<u32>> {
    std::fs::read_to_string(path)
    .expect("Failed to read input file.")
    .split("\n")
        .map(|line| line.to_string())
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
        )
        .collect()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_1() {
        let grid = file_to_grid("example.txt");
        visible_trees(grid);
    }
}
