use anyhow::Context;

struct Board {
    nums: [u32; 25],
    marks: [bool; 25],
}

impl Board {
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut nums = [0; 25];
        for (i, n) in s
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .enumerate()
        {
            nums[i] = n?;
        }
        Ok(Self {
            nums,
            marks: [false; 25],
        })
    }

    fn find_bingo(&self) -> bool {
        for i in (0..5).map(|n| n * 5) {
            let row = &self.marks[i..i + 5];
            if row.iter().all(|&b| b) {
                return true;
            }
        }
        for i in 0..5 {
            let mut column_indices = (0..5).map(|n| n * 5 + i);
            if column_indices.all(|n| self.marks[n]) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> u32 {
        self.nums
            .iter()
            .copied()
            .zip(self.marks.iter().copied())
            .filter_map(|(n, b)| (!b).then(|| n))
            .sum()
    }

    fn mark(&mut self, num: u32) {
        self.nums
            .iter()
            .copied()
            .zip(self.marks.iter_mut())
            .for_each(|(n, b)| *b |= n == num);
    }
}

fn parse(input: &str) -> anyhow::Result<(Vec<u32>, Vec<Board>)> {
    let mut sections = input.split("\n\n");
    let numbers = sections
        .next()
        .context("numbers required")?
        .split(',')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let boards = sections
        .map(|s| Board::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;

    Ok((numbers, boards))
}

fn part1(input: &str) -> anyhow::Result<()> {
    let (numbers, mut boards) = parse(input)?;

    let mut score = 0;

    'outer: for n in numbers {
        for board in boards.iter_mut() {
            board.mark(n);
            if board.find_bingo() {
                score = board.score() * n;
                break 'outer;
            }
        }
    }

    println!("{}", score);

    Ok(())
}

fn part2(input: &str) -> anyhow::Result<()> {
    use std::collections::HashSet;

    let (numbers, mut boards) = parse(input)?;

    let mut remaining = (0..boards.len()).collect::<HashSet<_>>();
    let mut score = 0;

    'outer: for n in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            board.mark(n);
            if remaining.contains(&i) && board.find_bingo() {
                remaining.remove(&i);
                if remaining.len() == 0 {
                    score = board.score() * n;
                    break 'outer;
                }
            }
        }
    }

    println!("{}", score);

    Ok(())
}

crate::parts!(part1 part2);
