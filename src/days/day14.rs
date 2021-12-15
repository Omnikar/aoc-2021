use std::collections::HashMap as Map;

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut letters = Vec::<u8>::new();
    let (template, rest) = input.trim().split_once("\n\n").unwrap();
    let mut get = |l| {
        letters
            .iter()
            .enumerate()
            .find_map(|(i, n)| (*n == l as u8).then(|| i))
            .unwrap_or_else(|| {
                letters.push(l);
                letters.len() - 1
            })
    };
    let map_contents = rest
        .trim()
        .lines()
        .map(|l| {
            let (pair, letter) = l.split_once(" -> ").unwrap();
            let mut pair = pair.chars();
            let a = get(pair.next().unwrap() as u8);
            let b = get(pair.next().unwrap() as u8);
            let letter = get(letter.chars().next().unwrap() as u8);
            ((a, b), letter)
        })
        .collect::<Vec<_>>();
    let template = template.bytes().map(|b| get(b)).collect();
    let mut map = vec![vec![0; letters.len()]; letters.len()];
    for ((a, b), n) in map_contents {
        map[a][b] = n;
    }
    (template, map)
}

fn grow(
    known: &mut Map<((usize, usize), u64), Vec<u64>>,
    map: &[Vec<usize>],
    (a, b): (usize, usize),
    count: u64,
) -> Vec<u64> {
    if let Some(freqs) = known.get(&((a, b), count)) {
        return freqs.to_owned();
    }
    let mut freqs = vec![0; map.len()];

    if count == 1 {
        freqs[a] += 1;
        let n = map[a][b];
        freqs[n] += 1;
    } else {
        let n = map[a][b];
        let new_freqs_0 = grow(known, map, (a, n), count - 1);
        let new_freqs_1 = grow(known, map, (n, b), count - 1);
        [new_freqs_0, new_freqs_1]
            .into_iter()
            .for_each(|new_freqs| {
                freqs
                    .iter_mut()
                    .zip(new_freqs.into_iter())
                    .for_each(|(n, count)| *n += count)
            });
    }

    known.entry(((a, b), count)).or_insert(freqs).to_owned()
}

fn simulate(input: &str, count: u64) -> u64 {
    let (template, map) = parse(input);

    let mut freqs = vec![0; map.len()];
    freqs[*template.last().unwrap()] = 1;

    for pair in template.windows(2) {
        let pair = (pair[0], pair[1]);
        let mut known = Map::new();
        let new_freqs = grow(&mut known, &map, pair, count);
        freqs
            .iter_mut()
            .zip(new_freqs.into_iter())
            .for_each(|(n, count)| *n += count);
    }

    let m_common = freqs.iter().copied().max().unwrap();
    let l_common = freqs.iter().copied().min().unwrap();
    m_common - l_common
}

fn part1(input: &str) {
    let ans = simulate(input, 10);
    println!("{}", ans);
}

fn part2(input: &str) {
    let ans = simulate(input, 40);
    println!("{}", ans);
}

crate::parts!(part1 part2);
