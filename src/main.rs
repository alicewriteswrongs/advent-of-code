#[macro_use]
extern crate anyhow;

mod day1;

fn main() -> anyhow::Result<()> {
    day1::solution()?;

    Ok(())
}
