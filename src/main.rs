use advent_solutions_2025::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("1 day solution: {}\n", crate::day1::solve()?.1);
    Ok(())
}
