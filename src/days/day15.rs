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

    fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut u32 {
        self.contents.get_mut(y * self.width + x).unwrap()
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

fn calc_cost(grid: Grid) -> u32 {
    let mut cost_grid = Grid {
        width: grid.width,
        contents: vec![u32::MAX; grid.contents.len()],
    };

    let mut changed = Vec::new();
    let (w, h) = (grid.width, grid.height());
    for pos in (0..w)
        .flat_map(|i| (0..=i).rev().zip(0..=i.min(h - 1)))
        .chain((1..h).flat_map(|i| (i..h).zip((0..w).rev())))
    {
        let cost;
        if pos == (0, 0) {
            cost = 0;
        } else {
            let adjs = cost_grid.adjs(pos);
            cost = adjs.map(|pos| cost_grid.get(pos)).min().unwrap_or(u32::MAX) + grid.get(pos);
        }
        changed.push(pos);
        *cost_grid.get_mut(pos) = cost;
    }
    while !changed.is_empty() {
        let mut to_check = std::mem::take(&mut changed)
            .into_iter()
            .flat_map(|pos| cost_grid.adjs(pos))
            .collect::<Vec<_>>();
        to_check.sort();
        to_check.dedup();
        for pos in to_check {
            let cost;
            if pos == (0, 0) {
                cost = 0;
            } else {
                let adjs = cost_grid.adjs(pos);
                cost = adjs.map(|pos| cost_grid.get(pos)).min().unwrap_or(u32::MAX) + grid.get(pos);
            }
            let bor = cost_grid.get_mut(pos);
            if *bor != cost {
                changed.push(pos);
                *bor = cost;
            }
        }
    }
    cost_grid.get((w - 1, h - 1))
}

fn part1(input: &str) {
    let risk = calc_cost(Grid::from_str(input));
    println!("{}", risk);
}

fn part2(input: &str) {
    let small_grid = Grid::from_str(input);
    let mut new_row = Vec::with_capacity(small_grid.contents.len() * 5);
    for i in 0..5 {
        if i == 0 {
            new_row = small_grid.contents.clone();
            continue;
        }
        new_row = new_row
            .chunks(small_grid.width * i as usize)
            .zip(
                small_grid
                    .contents
                    .iter()
                    .map(|n| (*n - 1 + i) % 9 + 1)
                    .collect::<Vec<_>>()
                    .chunks(small_grid.width),
            )
            .flat_map(|(row, nrow)| row.iter().copied().chain(nrow.iter().copied()))
            .collect();
    }
    let mut new = Vec::with_capacity(new_row.len() * 5);
    for i in 0..5 {
        new.extend(new_row.iter().map(|n| (*n - 1 + i) % 9 + 1));
    }
    let new_grid = Grid {
        width: small_grid.width * 5,
        contents: new,
    };

    let risk = calc_cost(new_grid);
    println!("{}", risk);
}

crate::parts!(part1 part2);
