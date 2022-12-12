use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
struct Tile {
    height: char,
    visited: bool,
    parent: Option<usize>,
}

impl Tile {
    fn new(height: char) -> Self {
        Self {
            height,
            visited: false,
            parent: None,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    start: usize,
    end: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let width = input.lines().last().map(|l| l.chars().count()).unwrap();
        let tiles = input.lines().flat_map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<char>>();
        let start = tiles.iter().position(|&c| c == 'S').unwrap();
        let end = tiles.iter().position(|&c| c == 'E').unwrap();
        let tiles = tiles.iter().enumerate().map(|(i,&x)| {
            if i == start {
                Tile::new('a')
            } else if i == end {
                Tile::new('z')
            } else {
                Tile::new(x)
            }
        }).collect::<Vec<Tile>>();

        // dbg!(&start, &end);

        Self {
            tiles,
            width,
            start,
            end,
        }
    }

    fn get_candidates(&mut self, idx: usize) -> Vec<usize> {
        let mut candidates: Vec<usize> = vec![];
        let current = self.tiles.get(idx).unwrap();
        let current_height = current.height;
        // left
        if idx % self.width > 0 {
            let candidate_idx = idx - 1;
            if let Some(tile) = self.tiles.get(candidate_idx) {
                if !tile.visited {
                    let tile_diff = tile.height as i32 - current_height as i32;
                    if tile_diff <= 1 {
                        // dbg!(current_height, tile.height, tile_diff);
                        candidates.push(candidate_idx);
                    }
                }
            }
        }
        // right
        let candidate_idx = idx + 1;
        if idx % self.width < self.width - 1 {
            if let Some(tile) = self.tiles.get(candidate_idx) {
                if !tile.visited {
                    let tile_diff = tile.height as i32 - current_height as i32;
                    if tile_diff <= 1 {
                        // dbg!(current_height, tile.height, tile_diff);
                        candidates.push(candidate_idx);
                    }
                }
            }
        }
        // above
        if idx > self.width {
            let candidate_idx = idx - self.width;
            if let Some(tile) = self.tiles.get(candidate_idx) {
                if !tile.visited {
                    let tile_diff = tile.height as i32 - current_height as i32;
                    if tile_diff <= 1 {
                        // dbg!(current_height, tile.height, tile_diff);
                        candidates.push(candidate_idx);
                    }
                }
            }
        }
        // below
        let candidate_idx = idx + self.width;
        if let Some(tile) = self.tiles.get(candidate_idx) {
            if !tile.visited {
                let tile_diff = tile.height as i32 - current_height as i32;
                if tile_diff <= 1 {
                    // dbg!(current_height, tile.height, tile_diff);
                    candidates.push(candidate_idx);
                }
            }
        }

        candidates
    }

    fn find_path(&mut self, start: usize, end: usize) -> Vec<usize> {
        let mut path: Vec<usize> = vec![];
        let mut queue = VecDeque::<usize>::new();
        self.tiles.get_mut(start).unwrap().visited = true;
        queue.push_back(start);
        while let Some(idx) = queue.pop_front() {
            // dbg!(&idx);
            if idx == end {
                let mut current_idx = Some(idx);
                while let Some(parent_idx) = current_idx {
                    // dbg!(&parent_idx);
                    current_idx = self.tiles.get(parent_idx).unwrap().parent;
                    path.push(parent_idx);
                }
                break;
            }
            let candidates = self.get_candidates(idx);
            for candidate_idx in candidates {
                if let Some(candidate) = self.tiles.get_mut(candidate_idx) {
                    candidate.visited = true;
                    candidate.parent = Some(idx);
                }
                queue.push_back(candidate_idx);
            }
        }

        path
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new(input);
    let path = map.find_path(map.start, map.end);
    // dbg!(&map, &path, &path.len());

    Some(path.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);
    let starting_points = map.tiles.iter().enumerate().filter_map(|(i, t)| {
        if t.height == 'a' {
            Some(i)
        } else {
            None
        }
    }).collect::<Vec<usize>>();
    let result = starting_points.iter().filter_map(|&start| {
        let mut map = Map::new(input);
        let path = map.find_path(start, map.end);
        if path.is_empty() {
            None
        } else {
            Some(path.len() as u32 - 1)
        }
    }).min().unwrap();
    dbg!(result);

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
