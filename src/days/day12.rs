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

fn explore(map: &Map<u16, Set<u16>>, loc: &Set<u16>, visited_smalls: Set<u16>, twice: bool) -> u32 {
    let mut stack = vec![(0, map, loc.iter(), visited_smalls, twice)];
    while let Some((count, map, loc, visited_smalls, twice)) = stack.last_mut() {
        if let Some(cave) = loc.next().copied() {
            if cave == END {
                *count += 1;
            } else {
                let visited_small = small(cave) && visited_smalls.contains(&cave);
                if !visited_small || *twice {
                    let mut new_visited_smalls = visited_smalls.clone();
                    if small(cave) {
                        new_visited_smalls.insert(cave);
                    }
                    let new_twice = *twice && !visited_small;
                    let args = (
                        0,
                        *map,
                        map.get(&cave).unwrap().iter(),
                        new_visited_smalls,
                        new_twice,
                    );
                    stack.push(args);
                }
            }
        } else {
            let count = *count;
            stack.pop();
            match stack.last_mut() {
                Some((outer_count, ..)) => *outer_count += count,
                _ => return count,
            }
        }
    }
    unreachable!()
}

fn part1(input: &str) {
    let map = parse(input);

    let start = map.get(&START).expect("missing start");
    let mut start_set = Set::new();
    start_set.insert(START);
    let count = explore(&map, start, start_set, false);

    println!("{}", count);
}

fn part2(input: &str) {
    let map = parse(input);

    let start = map.get(&START).expect("missing start");
    let mut start_set = Set::new();
    start_set.insert(START);
    let count = explore(&map, start, start_set, true);

    println!("{}", count);
}

crate::parts!(part1 part2);
