fn parse(input: &str) -> (u8, u8) {
    let mut lines = input.lines();
    lines
        .next()
        .and_then(|s| s.strip_prefix("Player 1 starting position: "))
        .and_then(|s| s.parse().ok())
        .and_then(|p1| {
            lines
                .next()
                .and_then(|s| s.strip_prefix("Player 2 starting position: "))
                .and_then(|s| s.parse().ok())
                .map(|p2| (p1, p2))
        })
        .unwrap()
}

// ap: active player
// pp: passive player

fn part1(input: &str) {
    let (mut p1, mut p2) = parse(input);
    let (mut p1s, mut p2s) = (0, 0);
    let mut tot = 0;
    let mut die = 0;
    let mut roll = || {
        die %= 100;
        die += 1;
        tot += 1;
        die
    };
    let mut p1t = true;
    let score = loop {
        let (ap, aps, pps) = if p1t {
            (&mut p1, &mut p1s, &p2s)
        } else {
            (&mut p2, &mut p2s, &p1s)
        };

        let mov: u8 = (0..3).map(|_| roll()).sum();
        *ap -= 1;
        *ap += mov;
        *ap %= 10;
        *ap += 1;
        *aps += *ap as u32;

        if *aps >= 1000 {
            break *pps;
        }

        p1t ^= true;
    };

    println!("{}", score * tot);
}

fn wins(ap: u8, pp: u8, aps: u8, pps: u8) -> (u64, u64) {
    if pps >= 21 {
        return (0, 1);
    }

    let mut out = (0, 0);
    // distribution of possible roll sums
    let rolls = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    for roll in rolls {
        let nap = (ap + roll.0 - 1) % 10 + 1;
        let naps = aps + nap;
        let nwins = wins(pp, nap, pps, naps);
        out.0 += nwins.1 * roll.1;
        out.1 += nwins.0 * roll.1;
    }
    out
}

fn part2(input: &str) {
    let (p1, p2) = parse(input);
    let (p1ws, p2ws) = wins(p1, p2, 0, 0);
    println!("{}", p1ws.max(p2ws));
}

crate::parts!(part1 part2);
