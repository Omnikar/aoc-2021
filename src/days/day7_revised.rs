use anyhow::Context;

fn parse(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().map_err(Into::into))
        .collect()
}

fn calc_fuel<F1, F2>(mut nums: Vec<u32>, f: F1) -> Option<u32>
where
    F1: Fn(u32) -> F2,
    F2: Fn(u32) -> u32,
{
    nums.sort_unstable();
    (nums[0]..nums[nums.len() - 1])
        .map(|i| nums.iter().copied().map(f(i)).sum())
        .min()
}

fn part1(input: &str) -> anyhow::Result<()> {
    let nums = parse(input)?;
    let fuel_cost = calc_fuel(nums, |i| move |n| n.max(i) - n.min(i)).context("no numbers")?;

    println!("{}", fuel_cost);

    Ok(())
}

fn part2(input: &str) -> anyhow::Result<()> {
    let nums = parse(input)?;
    let fuel_cost = calc_fuel(nums, |i| {
        move |n| {
            let diff = n.max(i) - n.min(i);
            (diff + 1) * diff / 2
        }
    })
    .context("no numbers")?;

    println!("{}", fuel_cost);

    Ok(())
}

crate::parts!(part1 part2);
