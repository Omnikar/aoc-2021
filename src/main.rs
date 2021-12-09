mod days;
mod macros;

use std::io::{stdin, Read};

fn main() {
    let mut args = std::env::args().skip(1).take(2);
    let day = args.next().expect("missing day");
    let part = args.next().expect("missing part");

    let func = *days::DAYS
        .get(day.as_str())
        .expect("day does not exist")
        .get(part.as_str())
        .expect("part does not exist");

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    func(&input);
}
