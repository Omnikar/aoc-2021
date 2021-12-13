struct Paper {
    width: usize,
    grid: Vec<bool>,
}

impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width;
        let s = self
            .grid
            .chunks(width)
            .map(|row| {
                row.iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Paper {
    fn from_str(s: &str) -> Self {
        let dots = s
            .trim()
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<usize>(), y.parse::<usize>()))
            .map(|(x, y)| (x.unwrap(), y.unwrap()));
        let (xs, ys): (Vec<_>, Vec<_>) = dots.clone().unzip();
        let (width, height) = (
            xs.into_iter().max().unwrap() + 1,
            ys.into_iter().max().unwrap() + 1,
        );

        let mut grid = vec![false; width * height];
        dots.for_each(|(x, y)| grid[y * width + x] = true);

        Self { width, grid }
    }

    fn transpose(&mut self) {
        let width = self.width;
        let height = self.grid.len() / width;
        let mut new_grid = vec![false; self.grid.len()];
        let rows = self.grid.chunks(width).collect::<Vec<_>>();
        new_grid
            .iter_mut()
            .enumerate()
            .for_each(|(i, dot)| *dot = rows[i % height][i / height]);
        self.width = height;
        self.grid = new_grid;
    }

    fn fold_x(&mut self, y: usize) {
        let width = self.width;
        let (remain_rows, fold_rows) = self.grid.split_at_mut(width * y);
        let fold_rows = &fold_rows[width..];
        let (remain_rows, fold_rows) = (remain_rows.chunks_mut(width), fold_rows.chunks(width));
        remain_rows.rev().zip(fold_rows).for_each(|(r_row, f_row)| {
            r_row
                .iter_mut()
                .zip(f_row.iter())
                .for_each(|(r, f)| *r |= f)
        });
        self.grid.truncate(width * y);
    }

    fn fold_y(&mut self, x: usize) {
        self.transpose();
        self.fold_x(x);
        self.transpose();
    }
}

fn parse(input: &str) -> (Paper, Vec<(bool, usize)>) {
    let (paper_str, folds_str) = input.split_once("\n\n").unwrap();
    let paper = Paper::from_str(paper_str);
    let folds = folds_str
        .trim()
        .lines()
        .map(|l| {
            let (dir, s) = l
                .strip_prefix("fold along ")
                .and_then(|s| s.split_once('='))
                .unwrap();
            (dir == "y", s.parse::<usize>().unwrap())
        })
        .collect();
    (paper, folds)
}

fn part1(input: &str) {
    let (mut paper, folds) = parse(input);
    if folds[0].0 {
        paper.fold_x(folds[0].1);
    } else {
        paper.fold_y(folds[0].1);
    }
    let count = paper.grid.into_iter().filter(|b| *b).count();
    println!("{:?}", count);
}

fn part2(input: &str) {
    let (mut paper, folds) = parse(input);
    folds.into_iter().for_each(|(x, n)| {
        if x {
            paper.fold_x(n);
        } else {
            paper.fold_y(n);
        }
    });
    println!("{}", paper);
}

crate::parts!(part1 part2);
