use std::collections::{HashMap as Map, HashSet as Set};

struct Grid {
    width: usize,
    contents: Vec<u32>,
}

impl Grid {
    fn height(&self) -> usize {
        self.contents.len() / self.width
    }

    fn get(&self, (x, y): (usize, usize)) -> u32 {
        *self.contents.get(y * self.width + x).unwrap()
    }

    fn from_str(s: &str) -> Self {
        let mut lines = s.trim().lines().peekable();
        let width = lines.peek().expect("empty input").len();
        let contents = lines
            .flat_map(|l| l.chars())
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .unwrap();
        Self { width, contents }
    }

    fn adjs(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width;
        let height = self.height();
        let hor = [0, 2]
            .into_iter()
            .filter_map(move |n| (x + n).checked_sub(1).filter(|n| *n < width))
            .zip(std::iter::repeat(y));
        let ver = std::iter::repeat(x).zip(
            [0, 2]
                .into_iter()
                .filter_map(move |n| (y + n).checked_sub(1).filter(|n| *n < height)),
        );
        hor.chain(ver)
    }
}

fn find_lows(grid: &Grid) -> Map<(usize, usize), Set<(usize, usize)>> {
    let mut low = Map::new();
    let mut checked = Map::new();

    for mut pos in (0..grid.height()).flat_map(|y| (0..grid.width).zip(std::iter::repeat(y))) {
        let mut new_checked = Set::new();
        let low_point = loop {
            if let Some(&p) = checked.get(&pos) {
                break p;
            }
            let min_adj = grid
                .adjs(pos)
                .map(|pos| (pos, grid.get(pos)))
                .min_by_key(|(_, val)| *val)
                .unwrap();
            new_checked.insert(pos);
            if min_adj.1 <= grid.get(pos) {
                pos = min_adj.0;
            } else {
                low.insert(pos, Set::new());
                break pos;
            }
        };
        new_checked.iter().for_each(|&pos| {
            checked.insert(pos, low_point);
        });
        new_checked.iter().for_each(|&pos| {
            low.get_mut(&low_point).unwrap().insert(pos);
        });
    }

    low
}

fn part1(input: &str) {
    let grid = Grid::from_str(input);
    let low = find_lows(&grid);
    let risk_level = low
        .keys()
        .map(|&pos| grid.get(pos))
        .map(|n| n + 1)
        .sum::<u32>();
    println!("{}", risk_level);
}

fn part2(input: &str) {
    let grid = Grid::from_str(input);
    let mut low = find_lows(&grid);
    low.values_mut()
        .for_each(|set| set.retain(|&pos| grid.get(pos) != 9));
    let mut basin_sizes = low.values().map(|set| set.len()).collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    let product = basin_sizes[(basin_sizes.len() - 3)..]
        .iter()
        .copied()
        .product::<usize>();
    println!("{}", product);
}

crate::parts!(part1 part2);
