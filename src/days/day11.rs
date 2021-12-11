struct Grid {
    contents: [u8; 100],
}

impl Grid {
    fn get(&self, (x, y): (usize, usize)) -> u8 {
        *self.contents.get(y * 10 + x).unwrap()
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut u8 {
        self.contents.get_mut(y * 10 + x).unwrap()
    }

    fn from_str(s: &str) -> Self {
        let mut contents = [0; 100];
        s.trim()
            .lines()
            .peekable()
            .flat_map(|l| l.chars())
            .map(|c| c.to_digit(10).expect("invalid digit"))
            .zip(contents.iter_mut())
            .for_each(|(n, n1)| *n1 = n as u8);
        Self { contents }
    }

    fn adjs(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let indices =
            |w: usize| (0..3).filter_map(move |n| (w + n).checked_sub(1).filter(|n| *n < 10));
        indices(x)
            .zip(std::iter::repeat(indices(y)))
            .flat_map(|(x, ys)| std::iter::repeat(x).zip(ys))
            .filter(move |pos| *pos != (x, y))
    }
}

fn simulate(mut grid: Grid, first_simul: bool) -> u32 {
    let mut flashes = 0;

    for i in 0.. {
        if !first_simul && i >= 100 {
            break;
        }
        grid.contents.iter_mut().for_each(|n| *n += 1);
        let mut flash = || {
            let mut flashed = false;
            for pos in (0..10).flat_map(|n| std::iter::repeat(n).zip(0..10)) {
                if (10..18).contains(&grid.get(pos)) {
                    *grid.get_mut(pos) = 18;
                    grid.adjs(pos).for_each(|pos| *grid.get_mut(pos) += 1);
                    flashes += 1;
                    flashed = true;
                }
            }
            flashed
        };
        while flash() {}
        if first_simul && grid.contents.iter().all(|n| *n > 9) {
            return i + 1;
        }
        grid.contents
            .iter_mut()
            .for_each(|n| *n = Some(*n).filter(|n| *n <= 9).unwrap_or(0));
    }

    flashes
}

fn part1(input: &str) {
    let grid = Grid::from_str(input);
    let flashes = simulate(grid, false);
    println!("{}", flashes);
}

fn part2(input: &str) {
    let grid = Grid::from_str(input);
    let first_simul = simulate(grid, true);
    println!("{}", first_simul);
}

crate::parts!(part1 part2);
