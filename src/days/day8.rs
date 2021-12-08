use anyhow::Context;

#[derive(Copy, Clone)]
struct Display(u8);
#[allow(dead_code)]
impl Display {
    fn a(&self) -> bool {
        self.0 & 1 == 1
    }
    fn b(&self) -> bool {
        self.0 >> 1 & 1 == 1
    }
    fn c(&self) -> bool {
        self.0 >> 2 & 1 == 1
    }
    fn d(&self) -> bool {
        self.0 >> 3 & 1 == 1
    }
    fn e(&self) -> bool {
        self.0 >> 4 & 1 == 1
    }
    fn f(&self) -> bool {
        self.0 >> 5 & 1 == 1
    }
    fn g(&self) -> bool {
        self.0 >> 6 & 1 == 1
    }
    fn digit(&self) -> Option<u8> {
        match self.0 {
            0b01110111 => Some(0),
            0b00100100 => Some(1),
            0b01011101 => Some(2),
            0b01101101 => Some(3),
            0b00101110 => Some(4),
            0b01101011 => Some(5),
            0b01111011 => Some(6),
            0b00100101 => Some(7),
            0b01111111 => Some(8),
            0b01101111 => Some(9),
            _ => None,
        }
    }
}
impl std::fmt::Debug for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Display({:07b})", self.0)
    }
}
impl std::str::FromStr for Display {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
        Ok(Self(num))
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<([Display; 10], [Display; 4])>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_once('|')
                .context("invalid entry")
                .map(|(dig_s, out_s)| {
                    (
                        dig_s.trim().split(' ').map(str::parse::<Display>),
                        out_s.trim().split(' ').map(str::parse::<Display>),
                    )
                })
                .map(|(dig_iter, out_iter)| {
                    let mut digits = [Display(0); 10];
                    let mut output = [Display(0); 4];
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
                .filter(|display| matches!(display.0.count_ones(), 2..=4 | 7))
        })
        .count();

    println!("{}", unique_counts);

    Ok(())
}

crate::parts!(part1);
