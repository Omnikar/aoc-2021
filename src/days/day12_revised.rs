use std::collections::{HashMap as Map, HashSet as Set};

const START: u16 = 0x7A7A;
const END: u16 = 0x7B7B;

fn parse(input: &str) -> Map<u16, Set<u16>> {
    let mut map = Map::<u16, Set<u16>>::new();
    for line in input.trim().lines() {
        let (from, to) = line.split_once('-').unwrap();
        fn parse_cave(s: &str) -> u16 {
            match s {
                "start" => START,
                "end" => END,
                s => {
                    let mut chars = s.chars();
                    let (c2, c1) = (chars.next().unwrap(), chars.next());
                    c2 as u16 + c1.map(|c| (c as u16) << 8).unwrap_or(0)
                }
            }
        }
        let (from, to) = (parse_cave(from), parse_cave(to));
        if from != END && to != START {
            map.entry(from).or_default().insert(to);
        }
        if to != END && from != START {
            map.entry(to).or_default().insert(from);
        }
    }
    map
}

fn small(cave: u16) -> bool {
    cave & u8::MAX as u16 >= b'a' as u16
}

#[derive(Clone, Copy)]
enum Smalls<'a> {
    None,
    Some { this: u16, last: &'a Smalls<'a> },
}

impl Smalls<'_> {
    fn contains(&self, cave: u16) -> bool {
        match self {
            Self::Some { this, last } => *this == cave || last.contains(cave),
            Self::None => false,
        }
    }
}

fn explore(map: &Map<u16, Set<u16>>, loc: &Set<u16>, visited_smalls: Smalls, twice: bool) -> u32 {
    loc.iter()
        .copied()
        .map(|cave| {
            if cave == END {
                1
            } else {
                let visited_small = small(cave) && visited_smalls.contains(cave);
                if !visited_small || twice {
                    let visited_smalls = if small(cave) {
                        Smalls::Some {
                            this: cave,
                            last: &visited_smalls,
                        }
                    } else {
                        visited_smalls
                    };
                    let new_twice = twice && !visited_small;
                    explore(map, map.get(&cave).unwrap(), visited_smalls, new_twice)
                } else {
                    0
                }
            }
        })
        .sum()
}

fn part1(input: &str) {
    let map = parse(input);

    let start = map.get(&START).expect("missing start");
    let count = explore(&map, start, Smalls::None, false);

    println!("{}", count);
}

fn part2(input: &str) {
    let map = parse(input);

    let start = map.get(&START).expect("missing start");
    let count = explore(&map, start, Smalls::None, true);

    println!("{}", count);
}

crate::parts!(part1 part2);
