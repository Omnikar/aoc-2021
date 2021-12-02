use anyhow::Context;

fn parse(input: &str) -> anyhow::Result<Vec<(&str, u32)>> {
    input
        .trim()
        .split('\n')
        .map(|s| {
            s.split_once(" ")
                .context("missing space")
                .and_then(|(dir, n)| n.parse::<u32>().map_err(|e| e.into()).map(|n| (dir, n)))
        })
        .collect()
}

fn part1(input: String) -> anyhow::Result<()> {
    let movements = parse(&input)?;

    let mut pos = (0, 0);

    for (dir, n) in movements {
        match dir {
            "forward" => pos.0 += n,
            "down" => pos.1 += n,
            "up" => pos.1 -= n,
            _ => anyhow::bail!("invalid direction"),
        }
    }

    println!("{}", pos.0 * pos.1);

    Ok(())
}

fn part2(input: String) -> anyhow::Result<()> {
    let movements = parse(&input)?;

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
            _ => anyhow::bail!("invalid direction"),
        }
    }

    println!("{}", pos.0 * pos.1);

    Ok(())
}

crate::parts!(part1 part2);
