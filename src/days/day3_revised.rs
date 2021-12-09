fn parse(input: &str) -> (Vec<u32>, u32) {
    let mut lines = input.trim().lines().peekable();
    let digit_count = lines.peek().expect("expected at least 1 number").len() as u32;
    let nums = lines
        .map(|s| u32::from_str_radix(s, 2))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    (nums, digit_count)
}

fn calc_gamma(nums: &[u32], digit_count: u32) -> u32 {
    let mut gamma = 0;
    for i in 0..digit_count {
        gamma += calc_gamma_digit(nums, i) << i;
    }
    gamma
}

fn calc_gamma_digit(nums: &[u32], digit: u32) -> u32 {
    (nums.iter().copied().map(|n| n >> digit & 1).sum::<u32>() * 2 >= nums.len() as u32)
        .then(|| 1u32)
        .unwrap_or(0)
}

fn part1(input: &str) {
    let (nums, digit_count) = parse(input);

    let gamma = calc_gamma(&nums, digit_count);
    let epsilon = !gamma & ((1 << digit_count) - 1);

    println!("{}", gamma * epsilon);
}

fn part2(input: &str) {
    let (nums, digit_count) = parse(input);

    let rating = |invert: bool| {
        let mut rating = 0;
        let mut new_nums = nums.clone();
        let mut i = digit_count;
        while let Some(&n) = new_nums.first() {
            rating = n;
            if i == 0 {
                break;
            }
            i -= 1;
            let gamma_digit = calc_gamma_digit(&new_nums, i) ^ invert as u32;
            new_nums.retain(move |&n| n >> i & 1 == gamma_digit);
        }
        rating
    };

    let o2_gen = rating(false);
    let co2_scrub = rating(true);

    println!("{}", o2_gen * co2_scrub);
}

crate::parts!(part1 part2);
