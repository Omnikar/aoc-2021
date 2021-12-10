fn is_opener(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn closing_pair(opener: char) -> char {
    match opener {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        c => c,
    }
}

fn corrupt_pair_score(closer: char) -> u64 {
    match closer {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn incomplete_pair_score(closer: char) -> u64 {
    match closer {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn process(input: &str) -> (u64, Vec<u64>) {
    let lines = input.trim().lines();

    let mut corrupt_score = 0;
    let mut incomplete_scores = Vec::new();

    'outer: for line in lines {
        let mut expected_closers = Vec::new();
        for c in line.chars() {
            if is_opener(c) {
                expected_closers.push(closing_pair(c));
            } else if expected_closers
                .last()
                .map(|&closer| c == closer)
                .unwrap_or(false)
            {
                expected_closers.pop();
            } else {
                corrupt_score += corrupt_pair_score(c);
                continue 'outer;
            }
        }
        if !expected_closers.is_empty() {
            let mut score = 0;
            for c in expected_closers.into_iter().rev() {
                score *= 5;
                score += incomplete_pair_score(c);
            }
            incomplete_scores.push(score);
        }
    }

    (corrupt_score, incomplete_scores)
}

fn part1(input: &str) {
    println!("{}", process(input).0);
}

fn part2(input: &str) {
    let mut scores = process(input).1;
    scores.sort_unstable();
    println!("{}", scores[scores.len() / 2]);
}

crate::parts!(part1 part2);
