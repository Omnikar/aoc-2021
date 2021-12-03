fn part1(input: &str) -> anyhow::Result<()> {
    let nums = input
        .trim()
        .split('\n')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let (count, _) = nums.into_iter().fold((0, u32::MAX), |(count, prev), next| {
        if next > prev {
            (count + 1, next)
        } else {
            (count, next)
        }
    });

    println!("{}", count);

    Ok(())
}

fn part2(input: &str) -> anyhow::Result<()> {
    let nums = input
        .trim()
        .split('\n')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let (count, _) = (0..nums.len() - 2)
        .map(|i| (nums[i], nums[i + 1], nums[i + 2]))
        .fold((0, u32::MAX), |(count, prev), next| {
            let sum = next.0 + next.1 + next.2;
            if sum > prev {
                (count + 1, sum)
            } else {
                (count, sum)
            }
        });

    println!("{}", count);

    Ok(())
}

crate::parts!(part1 part2);
