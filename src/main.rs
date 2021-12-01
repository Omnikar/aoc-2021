mod days;
mod macros;

use std::io::{stdin, Read};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn main() -> Result {
    let mut args = std::env::args().skip(1).take(2);
    let day = args.next().ok_or(Error("missing day"))?;
    let part = args.next().ok_or(Error("missing part"))?;

    let func = *days::DAYS
        .get(day.as_str())
        .ok_or(Error("day does not exist"))?
        .get(part.as_str())
        .ok_or(Error("part does not exist"))?;

    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    func(input)
}

struct Error(&'static str);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
impl std::error::Error for Error {}
