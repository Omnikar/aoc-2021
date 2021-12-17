#[derive(Debug)]
struct Packet {
    vers: u64,
    t_id: u64,
    content: Content,
}

#[derive(Debug)]
enum Content {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl Packet {
    fn from_str(s: &str) -> Self {
        let mut bits = s
            .trim()
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u64)
            .flat_map(|n| (0..4).rev().map(move |i| ((n >> i) & 1) == 1));
        Self::from_iter(&mut bits)
    }

    fn from_iter(iter: &mut impl Iterator<Item = bool>) -> Self {
        let vers = mk_num(iter, 3);
        let t_id = mk_num(iter, 3);
        if t_id == 4 {
            let mut num = 0;
            let mut cont = true;
            while cont {
                cont = iter.next().unwrap_or_default();
                num = (num << 4) + mk_num(iter, 4);
            }
            return Self {
                vers,
                t_id,
                content: Content::Literal(num),
            };
        }

        Self {
            vers,
            t_id,
            content: Content::Operator(if iter.next().unwrap() {
                let n_subs = mk_num(iter, 11);
                let mut subs = Vec::new();
                for _ in 0..n_subs {
                    subs.push(Self::from_iter(iter));
                }
                subs
            } else {
                let n_bits = mk_num(iter, 15);
                let mut subs = Vec::new();
                let mut iter = iter.take(n_bits as usize).collect::<Vec<_>>().into_iter();
                while iter.len() > 0 {
                    subs.push(Self::from_iter(&mut iter));
                }
                subs
            }),
        }
    }

    fn sum_vers(&self) -> u64 {
        match &self.content {
            Content::Literal(_) => self.vers,
            Content::Operator(packets) => {
                self.vers + packets.iter().map(|p| p.sum_vers()).sum::<u64>()
            }
        }
    }

    fn value(&self) -> u64 {
        let mut subs = match &self.content {
            Content::Literal(n) => return *n,
            Content::Operator(subs) => subs,
        }
        .iter()
        .map(|p| p.value());
        match self.t_id {
            0 => subs.sum(),
            1 => subs.product(),
            2 => subs.min().unwrap(),
            3 => subs.max().unwrap(),
            5 => (subs.next().unwrap() > subs.next().unwrap()) as u64,
            6 => (subs.next().unwrap() < subs.next().unwrap()) as u64,
            7 => (subs.next().unwrap() == subs.next().unwrap()) as u64,
            _ => unreachable!(),
        }
    }
}

fn mk_num(iter: &mut impl Iterator<Item = bool>, bits: u64) -> u64 {
    (0..bits)
        .rev()
        .zip(iter)
        .map(|(shft, n)| (n as u64) << shft)
        .sum()
}

fn part1(input: &str) {
    let packet = Packet::from_str(input);
    println!("{:?}", packet);
    println!("{}", packet.sum_vers());
}

fn part2(input: &str) {
    let packet = Packet::from_str(input);
    println!("{}", packet.value());
}

crate::parts!(part1 part2);
