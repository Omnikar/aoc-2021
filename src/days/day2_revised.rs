fn parse(input: &str) -> Vec<(&str, u32)> {
    input
        .trim()
        .split('\n')
        .map(|s| {
            let (dir, n) = s.split_once(" ").expect("missing space");
            n.parse::<u32>().map(|n| (dir, n)).unwrap()
        })
        .collect()
}

fn part1(input: &str) {
    let movements = parse(input);

    let mut pos = (0, 0);

    for (dir, n) in movements {
        match dir {
            "forward" => pos.0 += n,
            "down" => pos.1 += n,
            "up" => pos.1 -= n,
            _ => panic!("invalid direction"),
        }
    }

    println!("{}", pos.0 * pos.1);
}

fn part2(input: &str) {
    let movements = parse(input);

    let mut aim = 0;
    let mut pos = (0, 0);

    for (dir, n) in movements {
        match dir {
            "forward" => {
                pos.0 += n;
                pos.1 += aim * n;
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => panic!("invalid direction"),
        }
    }

    println!("{}", pos.0 * pos.1);
}

crate::parts!(part1 part2);
