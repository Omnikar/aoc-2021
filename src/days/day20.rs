use std::iter::{once, repeat};

struct Grid {
    width: usize,
    contents: Vec<bool>,
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.contents.chunks(self.width).try_for_each(|row| {
            row.iter()
                .try_for_each(|b| write!(f, "{}", if *b { '#' } else { '.' }))
                .and_then(|_| writeln!(f))
        })
    }
}

impl Grid {
    fn height(&self) -> usize {
        self.contents.len() / self.width
    }

    fn get(&self, (x, y): (usize, usize)) -> bool {
        *self.contents.get(y * self.width + x).unwrap()
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut bool {
        self.contents.get_mut(y * self.width + x).unwrap()
    }

    fn from_str(s: &str) -> Self {
        let mut lines = s.trim().lines().peekable();
        let width = lines.peek().expect("empty input").len();
        let contents = lines
            .flat_map(|l| l.chars())
            .map(|c| c == '#')
            .collect::<Vec<_>>();
        Self { width, contents }
    }

    fn adjs(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        (0..3).flat_map(move |n| (0..3).map(move |n| x + n - 1).zip(repeat(y + n - 1)))
    }

    fn expand(&mut self, border: bool) {
        let w = self.width;
        let nw = w + 2;
        let iter = repeat(border)
            .take(nw)
            .chain(
                self.contents
                    .chunks(w)
                    .map(|row| row.iter().copied())
                    .map(|row| once(border).chain(row).chain(once(border)))
                    .flatten(),
            )
            .chain(repeat(border).take(nw));
        self.contents = iter.collect();
        self.width = nw;
    }
}

fn parse(input: &str) -> (Vec<bool>, Grid) {
    let (alg, grid_str) = input.split_once("\n\n").unwrap();
    let alg = alg.trim().chars().map(|c| c == '#').collect();
    let grid = Grid::from_str(grid_str);
    (alg, grid)
}

fn enhance(alg: &[bool], grid: &mut Grid, count: u32) {
    grid.expand(false);
    let mut border = false;
    for _ in 0..count {
        let mut new_grid = Grid {
            contents: vec![false; grid.contents.len()],
            width: grid.width,
        };
        grid.expand(border);
        border = alg[if border { 511 } else { 0 }];
        new_grid.expand(border);
        let (w, h) = (grid.width, grid.height());
        for pos in (1..h - 1).flat_map(|y| (1..w - 1).zip(repeat(y))) {
            let num = grid
                .adjs(pos)
                .map(|pos| grid.get(pos))
                .fold(0, |n, b| (n << 1) + b as usize);
            *new_grid.get_mut(pos) = alg[num];
        }
        *grid = new_grid;
    }
}

fn part1(input: &str) {
    let (alg, mut grid) = parse(input);
    enhance(&alg, &mut grid, 2);
    let count = grid.contents.into_iter().filter(|b| *b).count();
    println!("{}", count);
}

fn part2(input: &str) {
    let (alg, mut grid) = parse(input);
    enhance(&alg, &mut grid, 50);
    let count = grid.contents.into_iter().filter(|b| *b).count();
    println!("{}", count);
}

crate::parts!(part1 part2);
