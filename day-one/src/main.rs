use std::error::Error;
use std::str::FromStr;

const DATA: &'static str = include_str!("./sample-data.txt");

fn main() -> Result<(), Box<dyn Error>> {
    dbg!(DATA
        .lines()
        .map(|s| i32::from_str(s).unwrap())
        .map(|mass| mass / 3 - 2)
        .sum::<i32>());

    Ok(())
}
