use anyhow::Context;

fn parse(input: &str) -> anyhow::Result<(Vec<u32>, u32)> {
    let mut lines = input.trim().lines().peekable();
    let digit_count = lines.peek().context("expected at least 1 number")?.len() as u32;
    let nums = lines
        .map(|s| u32::from_str_radix(s, 2))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((nums, digit_count))
}

fn calc_gamma(nums: &[u32], digit_count: u32) -> u32 {
    let mut gamma = 0;
    for i in 0..digit_count {
        gamma += (nums.iter().copied().map(|n| n >> i & 1).sum::<u32>() * 2 >= nums.len() as u32)
            .then(|| 1u32)
            .unwrap_or(0)
            * 1
            << i;
    }
    gamma
}

fn part1(input: &str) -> anyhow::Result<()> {
    let (nums, digit_count) = parse(input)?;

    let gamma = calc_gamma(&nums, digit_count);
    let epsilon = !gamma & (1 << digit_count) - 1;

    println!("{}", gamma * epsilon);

    Ok(())
}

fn part2(input: &str) -> anyhow::Result<()> {
    let (nums, digit_count) = parse(input)?;

    let rating = |invert: bool| {
        let mut gamma = calc_gamma(&nums, digit_count);
        if invert {
            gamma = !gamma & (1 << digit_count) - 1;
        }
        let mut rating = 0;
        let mut new_nums = nums.clone();
        let mut i = digit_count;
        while let Some(&n) = new_nums.first() {
            rating = n;
            if i == 0 {
                break;
            }
            i -= 1;
            new_nums.retain(move |&n| n >> i & 1 == gamma >> i & 1);
            gamma = calc_gamma(&new_nums, digit_count);
            if invert {
                gamma = !gamma & (1 << digit_count) - 1;
            }
        }
        rating
    };

    let o2_gen = rating(false);
    let co2_scrub = rating(true);

    println!("{}", o2_gen * co2_scrub);

    Ok(())
}

crate::parts!(part1 part2);
