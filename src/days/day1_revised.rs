fn parse(input: &str) -> Vec<u32> {
    input
        .trim()
        .split('\n')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn part1(input: &str) {
    let nums = parse(input);
    let count = nums.windows(2).filter(|w| w[1] > w[0]).count();

    println!("{}", count);
}

fn part2(input: &str) {
    let nums = parse(input);
    let count = nums.windows(4).filter(|w| w[3] > w[0]).count();

    println!("{}", count);
}

crate::parts!(part1 part2);
