use anyhow::Context;

enum Direction {
    Forward,
    Down,
    Up,
}

fn parse_direction(input: &str) -> anyhow::Result<(Direction, u32)> {
    let parts = input.split_once(" ").context("missing space")?;
    let direction = match parts.0 {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => anyhow::bail!("invalid direction"),
    };
    let count = parts.1.parse()?;
    Ok((direction, count))
}

fn parse(input: String) -> anyhow::Result<Vec<(Direction, u32)>> {
    input.trim().split('\n').map(parse_direction).collect()
}

fn part1(input: String) -> anyhow::Result<()> {
    let movements = parse(input)?;

    let mut pos = (0, 0);

    movements.into_iter().for_each(|(dir, n)| match dir {
        Direction::Forward => pos.0 += n,
        Direction::Down => pos.1 += n,
        Direction::Up => pos.1 -= n,
    });

    println!("{}", pos.0 * pos.1);

    Ok(())
}

fn part2(input: String) -> anyhow::Result<()> {
    let movements = parse(input)?;

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

    Ok(())
}

crate::parts!(part1 part2);
