mod days;
mod macros;

use anyhow::Context;
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1).take(2);
    let day = args.next().context("missing day")?;
    let part = args.next().context("missing part")?;

    let func = *days::DAYS
        .get(day.as_str())
        .context("day does not exist")?
        .get(part.as_str())
        .context("part does not exist")?;

    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    func(&input)
}
