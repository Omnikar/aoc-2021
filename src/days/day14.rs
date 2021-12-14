use std::collections::HashMap as Map;

fn parse(input: &str) -> (Vec<u8>, Map<(u8, u8), u8>) {
    let (template, rest) = input.trim().split_once("\n\n").unwrap();
    let template = template.into();
    let map = rest
        .trim()
        .lines()
        .map(|l| {
            let (pair, letter) = l.split_once(" -> ").unwrap();
            let mut pair = pair.chars();
            let a = pair.next().unwrap() as u8;
            let b = pair.next().unwrap() as u8;
            let letter = letter.chars().next().unwrap() as u8;
            ((a, b), letter)
        })
        .collect();
    (template, map)
}

fn simulate(input: &str, count: u32) -> u32 {
    let (mut template, map) = parse(input);

    for i in 0..count {
        println!("{}", i);
        let mut new = Vec::new();
        template.windows(2).rev().zip(1..).for_each(|(pair, i)| {
            let pair = (pair[0], pair[1]);
            if let Some(l) = map.get(&pair) {
                new.push((*l, template.len() - i));
            }
        });
        new.into_iter().for_each(|(n, i)| template.insert(i, n));
        println!("len: {}", template.len());
    }

    let mut freqs = Map::new();
    template
        .into_iter()
        .for_each(|n| *freqs.entry(n).or_insert(0) += 1);
    let m_common = freqs.iter().max_by_key(|(_, count)| *count).unwrap();
    let l_common = freqs.iter().min_by_key(|(_, count)| *count).unwrap();
    m_common.1 - l_common.1
}

fn part1(input: &str) {
    let ans = simulate(input, 10);
    println!("{}", ans);
}

// Current algorithm is too inefficient for part 2
// fn part2(input: &str) {
//     let ans = simulate(input, 40);
//     println!("{}", ans);
// }

crate::parts!(part1);
