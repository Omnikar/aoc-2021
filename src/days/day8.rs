use anyhow::Context;

fn digit(display: u8) -> Option<u8> {
    match display {
        0b1110111 => Some(0),
        0b0100100 => Some(1),
        0b1011101 => Some(2),
        0b1101101 => Some(3),
        0b0101110 => Some(4),
        0b1101011 => Some(5),
        0b1111011 => Some(6),
        0b0100101 => Some(7),
        0b1111111 => Some(8),
        0b1101111 => Some(9),
        _ => None,
    }
}

fn parse_display(s: &str) -> anyhow::Result<u8> {
    let mut num = 0;
    for c in s.trim().chars() {
        num += 1
            << match c {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                _ => anyhow::bail!("invalid character"),
            };
    }
    Ok(num)
}

fn parse(input: &str) -> anyhow::Result<Vec<([u8; 10], [u8; 4])>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_once('|')
                .context("invalid entry")
                .map(|(dig_s, out_s)| {
                    (
                        dig_s.trim().split(' ').map(parse_display),
                        out_s.trim().split(' ').map(parse_display),
                    )
                })
                .map(|(dig_iter, out_iter)| {
                    let mut digits = [0; 10];
                    let mut output = [0; 4];
                    digits
                        .iter_mut()
                        .zip(dig_iter)
                        .for_each(|d| drop(d.1.map(|d1| *d.0 = d1)));
                    output
                        .iter_mut()
                        .zip(out_iter)
                        .for_each(|o| drop(o.1.map(|o1| *o.0 = o1)));
                    (digits, output)
                })
        })
        .collect::<Result<Vec<_>, _>>()
}

fn part1(input: &str) -> anyhow::Result<()> {
    let entries = parse(input)?;

    let unique_counts = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .1
                .into_iter()
                .filter(|display| matches!(display.count_ones(), 2..=4 | 7))
        })
        .count();

    println!("{}", unique_counts);

    Ok(())
}

fn part2(input: &str) -> anyhow::Result<()> {
    let entries = parse(input)?;

    let mut total = 0;

    for entry in entries {
        let mut possible = [0b1111111; 7];

        let mut narrow = |num: u8, ns: &[usize]| {
            for n in possible
                .iter_mut()
                .enumerate()
                .filter_map(|(i, n)| (!ns.contains(&i)).then(|| n))
            {
                *n &= !num
            }
        };

        for display in entry.0.iter().copied() {
            match display.count_ones() {
                2 => narrow(display, &[2, 5]),
                3 => narrow(display, &[0, 2, 5]),
                4 => narrow(display, &[1, 2, 3, 5]),
                6 => narrow(!display, &[2, 3, 4]),
                _ => (),
            }
        }

        let mut prev = possible;

        loop {
            for (i, n) in possible.into_iter().enumerate() {
                if n.count_ones() == 1 {
                    possible
                        .iter_mut()
                        .enumerate()
                        .filter_map(|(i1, n1)| (i1 != i).then(|| n1))
                        .for_each(|n1| *n1 &= !n);
                }
            }
            if possible == prev {
                break;
            }
            prev = possible;
        }

        for n in possible {
            if n.count_ones() != 1 {
                anyhow::bail!("unable to determine configuration");
            }
        }

        let reconfigure = |n: u8| {
            let mut out = 0;
            for i in 0..7u8 {
                let shift = (possible[i as usize] as f32).log2() as u8;
                out += ((n >> shift) & 1) << i;
            }
            out
        };

        let output = entry
            .1
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, n)| {
                digit(reconfigure(n))
                    .context("invalid digit")
                    .map(|n| n as u32 * 10u32.pow(i as u32))
            })
            .sum::<Result<u32, _>>()?;
        total += output;
    }

    println!("{}", total);

    Ok(())
}

crate::parts!(part1 part2);
