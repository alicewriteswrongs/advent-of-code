#[macro_use]
extern crate anyhow;

mod day1;
mod day2;

fn main() -> anyhow::Result<()> {
    day1::solution()?;
    day2::solution()?;

    Ok(())
}
