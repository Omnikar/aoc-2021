#[derive(Clone)]
enum Num {
    One(u8),
    Pair([Box<Num>; 2]),
}

impl std::fmt::Debug for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::One(n) => n.fmt(f),
            Num::Pair([left, right]) => write!(f, "[")
                .and_then(|_| left.fmt(f))
                .and_then(|_| write!(f, ","))
                .and_then(|_| right.fmt(f))
                .and_then(|_| write!(f, "]")),
        }
    }
}

impl std::ops::Index<usize> for Num {
    type Output = Num;
    fn index(&self, index: usize) -> &Num {
        match self {
            Num::Pair(pair) => &pair[index],
            Num::One(_) => panic!("tried to index number"),
        }
    }
}
impl std::ops::IndexMut<usize> for Num {
    fn index_mut(&mut self, index: usize) -> &mut Num {
        match self {
            Num::Pair(pair) => &mut pair[index],
            Num::One(_) => panic!("tried to index number"),
        }
    }
}
impl std::ops::Index<&[usize]> for Num {
    type Output = Num;
    fn index(&self, index: &[usize]) -> &Num {
        let mut cur = self;
        for &i in index {
            cur = &cur[i];
        }
        cur
    }
}
impl std::ops::IndexMut<&[usize]> for Num {
    fn index_mut(&mut self, index: &[usize]) -> &mut Num {
        let mut cur = self;
        for &i in index {
            cur = &mut cur[i];
        }
        cur
    }
}

impl Num {
    fn from_str(s: &str) -> Num {
        if let Ok(n) = s.parse() {
            return Num::One(n);
        }

        let s = s
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap();
        let mut bracket_c = 0;
        let mut comma_i = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => bracket_c += 1,
                ']' => bracket_c -= 1,
                ',' if bracket_c == 0 => {
                    comma_i = i;
                    break;
                }
                _ => (),
            }
        }
        let (left, right) = s.split_at(comma_i);
        let (left, right) = (
            Box::new(Num::from_str(left)),
            Box::new(Num::from_str(&right[1..])),
        );
        let pair = [left, right];

        Num::Pair(pair)
    }

    fn add(self, other: Num) -> Num {
        let mut pair = Num::Pair([Box::new(self), Box::new(other)]);
        pair.reduce();
        pair
    }

    fn reduce(&mut self) {
        if let Num::One(_) = self {
            return;
        }
        'outer: loop {
            let mut stack = vec![0];
            while !stack.is_empty() {
                let cur = &mut self[&*stack];
                match cur {
                    Num::Pair(pair) if stack.len() == 4 => {
                        let mut iter = pair.iter().map(|n| match **n {
                            Num::One(ref n) => *n,
                            _ => panic!("improperly reduced number"),
                        });
                        let (left_n, right_n) = (iter.next().unwrap(), iter.next().unwrap());
                        self[&*stack] = Num::One(0);
                        let mut left_stack = stack.clone();
                        while let Some(0) = left_stack.last() {
                            left_stack.pop();
                        }
                        if !left_stack.is_empty() {
                            *left_stack.last_mut().unwrap() = 0;
                            loop {
                                match &mut self[&*left_stack] {
                                    Num::Pair(_) => left_stack.push(1),
                                    Num::One(n) => {
                                        *n += left_n;
                                        break;
                                    }
                                }
                            }
                        }
                        let mut right_stack = stack.clone();
                        while let Some(1) = right_stack.last() {
                            right_stack.pop();
                        }
                        if !right_stack.is_empty() {
                            *right_stack.last_mut().unwrap() = 1;
                            loop {
                                match &mut self[&*right_stack] {
                                    Num::Pair(_) => right_stack.push(0),
                                    Num::One(n) => {
                                        *n += right_n;
                                        break;
                                    }
                                }
                            }
                        }
                        continue 'outer;
                    }
                    Num::Pair(_) => stack.push(0),
                    Num::One(_) => loop {
                        match stack.last_mut() {
                            Some(1) => stack.pop(),
                            Some(n @ 0) => {
                                *n = 1;
                                break;
                            }
                            _ => break,
                        };
                    },
                }
            }
            stack = vec![0];
            while !stack.is_empty() {
                let cur = &mut self[&*stack];
                match cur {
                    num @ Num::One(_)
                        if *match num {
                            Num::One(ref n) => n,
                            _ => unreachable!(),
                        } >= 10 =>
                    {
                        let n = *match num {
                            Num::One(ref n) => n,
                            _ => unreachable!(),
                        };
                        *num =
                            Num::Pair([Box::new(Num::One(n / 2)), Box::new(Num::One((n + 1) / 2))]);
                        continue 'outer;
                    }
                    Num::Pair(_) => stack.push(0),
                    Num::One(_) => loop {
                        match stack.last_mut() {
                            Some(1) => stack.pop(),
                            Some(n @ 0) => {
                                *n = 1;
                                break;
                            }
                            _ => break,
                        };
                    },
                }
            }
            break;
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Num::One(n) => *n as u32,
            Num::Pair([l, r]) => l.magnitude() * 3 + r.magnitude() * 2,
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Num> + '_ {
    input.trim().lines().map(Num::from_str)
}

fn part1(input: &str) {
    let mut nums = parse(input);
    let mut sum = nums.next().unwrap();
    for num in nums {
        sum = sum.add(num);
    }
    println!("{}", sum.magnitude());
}

fn part2(input: &str) {
    let nums = parse(input).collect::<Vec<_>>();
    let mut max = 0;
    nums.iter()
        .flat_map(|num| std::iter::repeat(num).zip(nums.iter()))
        .for_each(|(n1, n2)| {
            max = max.max(n1.clone().add(n2.clone()).magnitude());
            max = max.max(n2.clone().add(n1.clone()).magnitude());
        });
    println!("{}", max);
}

crate::parts!(part1 part2);
