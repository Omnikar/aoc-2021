fn parse(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
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

fn part1(input: &str) {
    let nums = parse(input);
    let fuel_cost = calc_fuel(nums, |i| move |n| n.max(i) - n.min(i)).expect("no numbers");

    println!("{}", fuel_cost);
}

fn part2(input: &str) {
    let nums = parse(input);
    let fuel_cost = calc_fuel(nums, |i| {
        move |n| {
            let diff = n.max(i) - n.min(i);
            (diff + 1) * diff / 2
        }
    })
    .expect("no numbers");

    println!("{}", fuel_cost);
}

crate::parts!(part1 part2);
