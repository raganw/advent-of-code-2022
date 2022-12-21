#[derive(Debug, Default)]
struct Tree {
    height: u8,
    visible: bool,
    score_up: u32,
    score_down: u32,
    score_left: u32,
    score_right: u32,
}

impl Tree {
    fn total_score(&self) -> u32 {
        self.score_up * self.score_down * self.score_left * self.score_right
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<Tree>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    let height = c.to_string().parse::<u8>().unwrap();
                    Tree {
                        height,
                        ..Tree::default()
                    }
                })
                .collect()
        })
        .collect();
    dbg!(&grid);

    let grid_width = grid.get(0).unwrap().len();
    let grid_height = grid.len();
    dbg!(grid_width, grid_height);

    // left side
    for row in grid.iter_mut() {
        let mut tallest: u8 = 0;
        for mut tree in row.iter_mut() {
            if tree.height > tallest {
                tallest = tree.height;
                tree.visible = true;
            }
        }
    }
    // right side
    for row in grid.iter_mut() {
        let mut tallest: u8 = 0;
        for mut tree in row.iter_mut().rev() {
            if tree.height > tallest {
                tallest = tree.height;
                tree.visible = true;
            }
        }
    }

    // top
    for col_idx in 0..grid_width {
        let mut tallest: u8 = 0;
        for (row_idx, row) in grid.iter_mut().enumerate() {
            if let Some(mut tree) = row.get_mut(col_idx) {
                if tree.height > tallest {
                    tallest = tree.height;
                    tree.visible = true;
                }
                if col_idx == 0
                    || col_idx == grid_width - 1
                    || row_idx == 0
                    || row_idx == grid_height - 1
                {
                    tree.visible = true;
                }
            } else {
                unreachable!("here?");
            }
        }
    }

    // bottom
    for x in 0..grid_width {
        let mut tallest: u8 = 0;
        for line in grid.iter_mut().rev() {
            if let Some(mut tree) = line.get_mut(x) {
                if tree.height > tallest {
                    tallest = tree.height;
                    tree.visible = true;
                }
            } else {
                unreachable!("here?");
            }
        }
    }
    //dbg!(&grid);
    let mut result: u32 = 0;
    for line in grid {
        for tree in line {
            if tree.visible {
                result += 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lookup_grid: Vec<Vec<Tree>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    let height = c.to_string().parse::<u8>().unwrap();
                    Tree {
                        height,
                        ..Tree::default()
                    }
                })
                .collect()
        })
        .collect();
    let mut grid: Vec<Vec<Tree>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    let height = c.to_string().parse::<u8>().unwrap();
                    Tree {
                        height,
                        ..Tree::default()
                    }
                })
                .collect()
        })
        .collect();

    for (row_idx, row) in grid.iter_mut().enumerate() {
        for (col_idx, mut tree) in row.iter_mut().enumerate() {
            let row = lookup_grid.get(row_idx).unwrap();
            // right
            for t in row.iter().skip(col_idx + 1) {
                tree.score_right += 1;
                if t.height >= tree.height {
                    break;
                }
            }
            // left
            for t in row.iter().take(col_idx).rev() {
                tree.score_left += 1;
                if t.height >= tree.height {
                    break;
                }
            }
            // down
            for r in lookup_grid.iter().skip(row_idx + 1) {
                tree.score_down += 1;
                let t = r.get(col_idx).unwrap();
                if t.height >= tree.height {
                    break;
                }
            }
            // up
            for r in lookup_grid.iter().take(row_idx).rev() {
                tree.score_up += 1;
                let t = r.get(col_idx).unwrap();
                if t.height >= tree.height {
                    break;
                }
            }
        }
    }

    dbg!(&grid);

    let mut result: u32 = 0;
    for row in grid {
        for tree in row {
            if result < tree.total_score() {
                result = tree.total_score();
            }
        }
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
