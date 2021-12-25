use std::iter::repeat;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    East,
    South,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    contents: Vec<Option<Dir>>,
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.contents.chunks(self.width).try_for_each(|l| {
            l.iter()
                .try_for_each(|c| {
                    write!(
                        f,
                        "{}",
                        match c {
                            Some(Dir::East) => '>',
                            Some(Dir::South) => 'v',
                            None => '.',
                        }
                    )
                })
                .and_then(|_| writeln!(f))
        })
    }
}

impl Grid {
    fn from_str(s: &str) -> Self {
        let mut lines = s.trim().lines().peekable();
        let width = lines.peek().expect("empty input").len();
        let contents = lines
            .flat_map(str::chars)
            .map(|c| match c {
                '>' => Some(Dir::East),
                'v' => Some(Dir::South),
                _ => None,
            })
            .collect();
        Self { width, contents }
    }

    fn height(&self) -> usize {
        self.contents.len() / self.width
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<Dir> {
        self.contents[y * self.width + x]
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut Option<Dir> {
        &mut self.contents[y * self.width + x]
    }

    fn step(&mut self, dir: Dir) -> bool {
        let mut delta = Vec::new();
        let (w, h) = (self.width, self.height());
        for pos in (0..h).flat_map(|y| (0..w).zip(repeat(y))) {
            if self.get(pos) == Some(dir) {
                let mut n_pos = pos;
                match dir {
                    Dir::East => n_pos.0 = (n_pos.0 + 1) % w,
                    Dir::South => n_pos.1 = (n_pos.1 + 1) % h,
                }
                if self.get(n_pos).is_none() {
                    delta.push((pos, None));
                    delta.push((n_pos, Some(dir)));
                }
            }
        }
        let moved = !delta.is_empty();
        delta
            .into_iter()
            .for_each(|(pos, val)| *self.get_mut(pos) = val);
        moved
    }
}

fn part1(input: &str) {
    let mut grid = Grid::from_str(input);
    let mut count = 0;
    while {
        count += 1;
        let mut moved = false;
        moved |= grid.step(Dir::East);
        moved |= grid.step(Dir::South);
        moved
    } {}
    println!("{}", count);
}

crate::parts!(part1);
