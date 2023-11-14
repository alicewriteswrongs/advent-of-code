#[macro_use]
extern crate anyhow;

mod day1;
mod day2;
mod day3;
mod lib;

fn main() -> anyhow::Result<()> {
    day1::solution()?;
    day2::solution()?;
    day3::solution()?;

    Ok(())
}
