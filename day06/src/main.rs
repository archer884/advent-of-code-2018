mod field;
mod point;

use crate::point::Point;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let points: Result<Vec<_>, _> = grabinput::from_stdin()
        .map(|x| x.trim().parse::<Point>())
        .collect();

    let witness_points = points?;
    let candidate_points = field::field_iterator(&witness_points).ok_or("Field is unbounded")?;

    let area_size = candidate_points
        .filter(|&x| witness_points.iter().map(|&y| x.distance(y)).sum::<i32>() < 10_000)
        .count();

    println!("{}", area_size);
    Ok(())
}
