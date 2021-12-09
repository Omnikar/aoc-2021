use std::collections::HashMap;

struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    fn from_str(s: &str) -> Self {
        let vals = s
            .split(" -> ")
            .flat_map(|s| s.split(','))
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let get = |i: usize| vals.get(i).copied().expect("invalid input");
        Self {
            start: (get(0), get(1)),
            end: (get(2), get(3)),
        }
    }

    fn is_hor(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_ver(&self) -> bool {
        self.start.0 == self.end.0
    }
}

fn parse(input: &str) -> Vec<Line> {
    input.lines().map(Line::from_str).collect()
}

fn dual_range(a: u32, b: u32) -> Box<dyn Iterator<Item = u32>> {
    if a < b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

fn coords_from_line(line: Line, count_diags: bool) -> Box<dyn Iterator<Item = (u32, u32)>> {
    match (line.is_hor(), line.is_ver(), count_diags) {
        (true, true, _) => Box::new(std::iter::once(line.start)),
        (true, false, _) => {
            Box::new(dual_range(line.start.0, line.end.0).zip(std::iter::repeat(line.start.1)))
        }
        (false, true, _) => {
            Box::new(std::iter::repeat(line.start.0).zip(dual_range(line.start.1, line.end.1)))
        }
        (false, false, true) => {
            Box::new(dual_range(line.start.0, line.end.0).zip(dual_range(line.start.1, line.end.1)))
        }
        (false, false, false) => Box::new(std::iter::empty()),
    }
}

fn part1(input: &str) {
    let mut points = HashMap::<(u32, u32), u32>::new();

    for line in parse(input) {
        coords_from_line(line, false).for_each(|coord| *points.entry(coord).or_insert(0) += 1);
    }

    let count = points.values().filter(|v| **v >= 2).count();

    println!("{}", count);
}

fn part2(input: &str) {
    let mut points = HashMap::<(u32, u32), u32>::new();

    for line in parse(input) {
        coords_from_line(line, true).for_each(|coord| *points.entry(coord).or_insert(0) += 1);
    }

    let count = points.values().filter(|v| **v >= 2).count();

    println!("{}", count);
}

crate::parts!(part1 part2);
