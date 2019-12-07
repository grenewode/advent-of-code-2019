use std::error::Error;
use std::str::FromStr;

const DATA: &'static str = include_str!("./sample-data.txt");

fn fuel_cost(mass: i32) -> i32 {
    let mut additional_mass = mass;
    let mut total_fuel_cost = 0;

    loop {
        let additional_fuel = additional_mass / 3 - 2;

        if additional_fuel <= 0 {
            break total_fuel_cost;
        }

        total_fuel_cost += additional_fuel;
        additional_mass = additional_fuel;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "day 1 part 1 = {}",
        DATA.lines()
            .map(|s| i32::from_str(s).unwrap())
            .map(|mass| mass / 3 - 2)
            .sum::<i32>()
    );

    println!(
        "day 1 part 2 = {}",
        DATA.lines()
            .map(|s| i32::from_str(s).unwrap())
            .map(fuel_cost)
            .sum::<i32>()
    );

    Ok(())
}
