enum Direction {
    Forward,
    Down,
    Up,
}

fn parse_direction(input: &str) -> (Direction, u32) {
    let parts = input.split_once(" ").expect("missing space");
    let direction = match parts.0 {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => panic!("invalid direction"),
    };
    let count = parts.1.parse().unwrap();
    (direction, count)
}

fn parse(input: &str) -> Vec<(Direction, u32)> {
    input.trim().split('\n').map(parse_direction).collect()
}

fn part1(input: &str) {
    let movements = parse(input);

    let mut pos = (0, 0);

    movements.into_iter().for_each(|(dir, n)| match dir {
        Direction::Forward => pos.0 += n,
        Direction::Down => pos.1 += n,
        Direction::Up => pos.1 -= n,
    });

    println!("{}", pos.0 * pos.1);
}

fn part2(input: &str) {
    let movements = parse(input);

    let mut aim = 0;
    let mut pos = (0, 0);

    movements.into_iter().for_each(|(dir, n)| match dir {
        Direction::Forward => {
            pos.0 += n;
            pos.1 += aim * n;
        }
        Direction::Down => aim += n,
        Direction::Up => aim -= n,
    });

    println!("{}", pos.0 * pos.1);
}

crate::parts!(part1 part2);
