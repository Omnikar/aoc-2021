fn parse(input: &str) -> anyhow::Result<[u64; 9]> {
    let mut fish = [0; 9];
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().map(|n| fish[n] += 1))
        .collect::<Result<(), _>>()?;
    Ok(fish)
}

fn simulate(input: &str, days: u32) -> anyhow::Result<u64> {
    let mut fish = parse(input)?;

    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    let total = fish.into_iter().sum();

    Ok(total)
}

fn part1(input: &str) -> anyhow::Result<()> {
    simulate(input, 80).map(|n| println!("{}", n))
}

fn part2(input: &str) -> anyhow::Result<()> {
    simulate(input, 256).map(|n| println!("{}", n))
}

crate::parts!(part1 part2);
