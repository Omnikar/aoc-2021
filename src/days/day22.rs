use std::iter::once;

type Range = [std::ops::RangeInclusive<i32>; 3];

struct Grid {
    on: Vec<Range>,
}

impl Grid {
    fn set(&mut self, range: Range, val: bool) {
        if val {
            let mut cur_vec = vec![range];
            for on in self.on.iter().cloned() {
                let mut new_cur_vec = Vec::new();
                for cur in cur_vec {
                    let (_, _, mut new_cur) = intersect(on.clone(), cur);
                    new_cur_vec.append(&mut new_cur);
                }
                cur_vec = new_cur_vec;
            }
            self.on.append(&mut cur_vec);
        } else {
            let mut new_on_vec = Vec::new();
            let on_vec = std::mem::take(&mut self.on);
            for on in on_vec {
                let (_, mut new_on, _) = intersect(on, range.clone());
                new_on_vec.append(&mut new_on);
            }
            self.on = new_on_vec;
        }
    }
}

fn intersect(r1: Range, r2: Range) -> (Option<Range>, Vec<Range>, Vec<Range>) {
    let x_int = (*r1[0].start()).max(*r2[0].start())..=(*r1[0].end()).min(*r2[0].end());
    let y_int = (*r1[1].start()).max(*r2[1].start())..=(*r1[1].end()).min(*r2[1].end());
    let z_int = (*r1[2].start()).max(*r2[2].start())..=(*r1[2].end()).min(*r2[2].end());
    let int = [x_int, y_int, z_int];
    if int.iter().any(std::ops::RangeInclusive::is_empty) {
        return (None, vec![r1], vec![r2]);
    }
    let r1xs = [
        *r1[0].start()..=*int[0].start() - 1,
        *int[0].end() + 1..=*r1[0].end(),
    ]
    .into_iter()
    .chain(once(int[0].clone()));
    let r2xs = [
        *r2[0].start()..=*int[0].start() - 1,
        *int[0].end() + 1..=*r2[0].end(),
    ]
    .into_iter()
    .chain(once(int[0].clone()));
    let r1ys = [
        *r1[1].start()..=*int[1].start() - 1,
        *int[1].end() + 1..=*r1[1].end(),
    ]
    .into_iter()
    .chain(once(int[1].clone()));
    let r2ys = [
        *r2[1].start()..=*int[1].start() - 1,
        *int[1].end() + 1..=*r2[1].end(),
    ]
    .into_iter()
    .chain(once(int[1].clone()));
    let r1zs = [
        *r1[2].start()..=*int[2].start() - 1,
        *int[2].end() + 1..=*r1[2].end(),
    ]
    .into_iter()
    .chain(once(int[2].clone()));
    let r2zs = [
        *r2[2].start()..=*int[2].start() - 1,
        *int[2].end() + 1..=*r2[2].end(),
    ]
    .into_iter()
    .chain(once(int[2].clone()));

    let r1_non_ints = r1zs
        .flat_map(|z| {
            r1ys.clone()
                .flat_map(|y| r1xs.clone().map(move |x| [x, y.clone()]))
                .map(move |[x, y]| [x, y, z.clone()])
        })
        .filter(|r| *r != int)
        .filter(|r| !r.iter().any(std::ops::RangeInclusive::is_empty))
        .collect::<Vec<_>>();
    let r2_non_ints = r2zs
        .flat_map(|z| {
            r2ys.clone()
                .flat_map(|y| r2xs.clone().map(move |x| [x, y.clone()]))
                .map(move |[x, y]| [x, y, z.clone()])
        })
        .filter(|r| *r != int)
        .filter(|r| !r.iter().any(std::ops::RangeInclusive::is_empty))
        .collect::<Vec<_>>();

    (Some(int), r1_non_ints, r2_non_ints)
}

fn parse(input: &str) -> impl Iterator<Item = (bool, Range)> + '_ {
    input.trim().lines().map(|l| {
        let (on, coords) = l.split_once(' ').unwrap();
        let on = on == "on";
        let mut coords = coords
            .split(',')
            .zip(["x=", "y=", "z="].into_iter())
            .map(|(s, pfx)| s.strip_prefix(pfx).unwrap().split_once("..").unwrap())
            .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
            .map(|(start, end)| start..=end);
        let xr = coords.next().unwrap();
        let yr = coords.next().unwrap();
        let zr = coords.next().unwrap();
        (on, [xr, yr, zr])
    })
}

fn solve(input: &str, bound_50: bool) -> u64 {
    let mut grid = Grid { on: Vec::new() };

    for (on, [xr, yr, zr]) in parse(input) {
        let range = if bound_50 {
            let xr = (*xr.start()).max(-50)..=(*xr.end()).min(50);
            let yr = (*yr.start()).max(-50)..=(*yr.end()).min(50);
            let zr = (*zr.start()).max(-50)..=(*zr.end()).min(50);
            let range = [xr, yr, zr];
            if range.iter().any(std::ops::RangeInclusive::is_empty) {
                continue;
            }
            range
        } else {
            [xr, yr, zr]
        };
        grid.set(range, on);
    }

    grid.on
        .into_iter()
        .map(|[xr, yr, zr]| {
            (*xr.end() - *xr.start() + 1) as u64
                * (*yr.end() - *yr.start() + 1) as u64
                * (*zr.end() - *zr.start() + 1) as u64
        })
        .sum()
}

fn part1(input: &str) {
    println!("{}", solve(input, true));
}

fn part2(input: &str) {
    println!("{}", solve(input, false));
}

crate::parts!(part1 part2);
