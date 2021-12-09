fn parse(input: &str) -> [u64; 9] {
    let mut fish = [0; 9];
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().map(|n| fish[n] += 1))
        .collect::<Result<(), _>>()
        .unwrap();
    fish
}

fn simulate(input: &str, days: u32) -> u64 {
    let mut fish = parse(input);

    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    let total = fish.into_iter().sum();

    total
}

fn part1(input: &str) {
    println!("{}", simulate(input, 80));
}

fn part2(input: &str) {
    println!("{}", simulate(input, 256));
}

crate::parts!(part1 part2);
