use itertools::Itertools;

use shared::input::AocBufReader;

struct Forest {
    trees: Vec<Vec<u8>>,
    n_rows: usize,
    n_cols: usize,
}

impl Forest {
    fn from_reader(reader: AocBufReader) -> Forest {
        let mut trees: Vec<Vec<u8>> = Vec::new();
        for row in reader {
            trees.push(row.chars().map(|c| c as u8 - 48).collect());
        }
        let n_rows = trees.len();
        let n_cols = trees[0].len();

        Forest {
            trees,
            n_rows,
            n_cols,
        }
    }

    fn is_tree_visible(&self, row_idx: usize, col_idx: usize) -> bool {
        if row_idx == 0 || row_idx == self.n_rows - 1 || col_idx == 0 || col_idx == self.n_cols - 1
        {
            // tree is on the outside!
            return true;
        }

        let tree_height: u8 = self.trees[row_idx][col_idx];
        // we can safely unwrap because we've already ruled out the edge trees
        let max_tree_west_height: u8 = *self.trees[row_idx][0..col_idx].iter().max().unwrap();
        let max_tree_east_height: u8 = *self.trees[row_idx][col_idx + 1..].iter().max().unwrap();
        let max_tree_north_height: u8 = self.trees[0..row_idx]
            .iter()
            .map(|row| row[col_idx])
            .max()
            .unwrap();
        let max_tree_south_height: u8 = self.trees[row_idx + 1..]
            .iter()
            .map(|row| row[col_idx])
            .max()
            .unwrap();

        if tree_height > max_tree_west_height
            || tree_height > max_tree_east_height
            || tree_height > max_tree_north_height
            || tree_height > max_tree_south_height
        {
            return true;
        }

        false
    }

    fn n_visible_trees(&self) -> usize {
        (0..self.n_rows)
            .cartesian_product(0..self.n_cols)
            .filter(|(row_idx, col_idx)| self.is_tree_visible(*row_idx, *col_idx))
            .count()
    }

    fn _ntrees_visible_west(&self, row_idx: usize, col_idx: usize) -> usize {
        if col_idx == 0 {
            return 0;
        }

        let tree_height: u8 = self.trees[row_idx][col_idx];
        for (idx, height) in self.trees[row_idx][0..col_idx].iter().rev().enumerate() {
            if *height >= tree_height {
                return idx + 1;
            }
        }
        return col_idx;
    }

    fn _ntrees_visible_east(&self, row_idx: usize, col_idx: usize) -> usize {
        if col_idx == self.n_cols - 1 {
            return 0;
        }

        let tree_height: u8 = self.trees[row_idx][col_idx];
        for (idx, height) in self.trees[row_idx][col_idx + 1..].iter().enumerate() {
            if *height >= tree_height {
                return idx + 1;
            }
        }
        return self.n_cols - col_idx - 1;
    }

    fn _ntrees_visible_north(&self, row_idx: usize, col_idx: usize) -> usize {
        if row_idx == 0 {
            return 0;
        }

        let tree_height: u8 = self.trees[row_idx][col_idx];
        for (ith_tree, row_to_check) in (0..row_idx).rev().enumerate() {
            let height_to_check: u8 = self.trees[row_to_check][col_idx];
            if height_to_check >= tree_height {
                return ith_tree + 1;
            }
        }
        row_idx
    }

    fn _ntrees_visible_south(&self, row_idx: usize, col_idx: usize) -> usize {
        if row_idx == self.n_rows - 1 {
            return 0;
        }

        let tree_height: u8 = self.trees[row_idx][col_idx];
        for (ith_tree, row_to_check) in (row_idx + 1..self.n_rows).enumerate() {
            let height_to_check: u8 = self.trees[row_to_check][col_idx];
            if height_to_check >= tree_height {
                return ith_tree + 1;
            }
        }
        self.n_rows - row_idx - 1
    }

    fn tree_score(&self, row_idx: usize, col_idx: usize) -> usize {
        self._ntrees_visible_west(row_idx, col_idx)
            * self._ntrees_visible_east(row_idx, col_idx)
            * self._ntrees_visible_north(row_idx, col_idx)
            * self._ntrees_visible_south(row_idx, col_idx)
    }

    fn max_trees_visible(&self) -> usize {
        (0..self.n_rows)
            .cartesian_product(0..self.n_cols)
            .map(|(row_idx, col_idx)| self.tree_score(row_idx, col_idx))
            .max()
            .unwrap()
    }
}

fn main() {
    let forest = Forest::from_reader(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{}", forest.n_visible_trees());
    println!("{}", forest.max_trees_visible());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_score_utilities() {
        let forest = Forest::from_reader(AocBufReader::from_string("inputs/example.txt"));
        assert_eq!(forest._ntrees_visible_north(1, 2), 1);
        assert_eq!(forest._ntrees_visible_south(1, 2), 2);
        assert_eq!(forest._ntrees_visible_west(1, 2), 1);
        assert_eq!(forest._ntrees_visible_east(1, 2), 2);
    }
}
