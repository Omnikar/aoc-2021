use std::iter::repeat;

struct Grid {
    contents: Box<[bool; 1_00_00_00]>,
}

impl Grid {
    fn get_mut(&mut self, (x, y, z): (i32, i32, i32)) -> &mut bool {
        let (x, y, z) = ((x + 50) as usize, (y + 50) as usize, (z + 50) as usize);
        &mut self.contents[z * 1_00_00 + y * 1_00 + x]
    }
}

fn parse(input: &str) -> impl Iterator<Item = (bool, [std::ops::RangeInclusive<i32>; 3])> + '_ {
    input.trim().lines().map(|l| {
        let (on, coords) = l.split_once(' ').unwrap();
        let on = on == "on";
        let mut coords = coords
            .split(',')
            .zip(["x=", "y=", "z="].into_iter())
            .map(|(s, pfx)| s.strip_prefix(pfx).unwrap().split_once("..").unwrap())
            .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
            .map(|(start, end): (i32, i32)| (start.max(-50), end.min(50)))
            .map(|(start, end)| start..=end);
        let xr = coords.next().unwrap();
        let yr = coords.next().unwrap();
        let zr = coords.next().unwrap();
        (on, [xr, yr, zr])
    })
}

fn part1(input: &str) {
    let mut grid = Grid {
        contents: Box::new([false; 1_00_00_00]),
    };

    for (on, [xr, yr, zr]) in parse(input) {
        for ((x, y), z) in zr.flat_map(|z| {
            yr.clone()
                .flat_map(|y| xr.clone().zip(repeat(y)))
                .zip(repeat(z))
        }) {
            *grid.get_mut((x, y, z)) = on;
        }
    }

    let count = grid.contents.iter().filter(|b| **b).count();
    println!("{}", count);
}

crate::parts!(part1);
