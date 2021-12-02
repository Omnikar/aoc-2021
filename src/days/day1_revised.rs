fn parse(input: String) -> Result<Vec<u32>, std::num::ParseIntError> {
    input
        .trim()
        .split('\n')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
}

fn part1(input: String) -> anyhow::Result<()> {
    let nums = parse(input)?;
    let count = nums.windows(2).filter(|w| w[1] > w[0]).count();

    println!("{}", count);

    Ok(())
}

fn part2(input: String) -> anyhow::Result<()> {
    let nums = parse(input)?;
    let count = nums.windows(4).filter(|w| w[3] > w[0]).count();

    println!("{}", count);

    Ok(())
}

crate::parts!(part1 part2);
