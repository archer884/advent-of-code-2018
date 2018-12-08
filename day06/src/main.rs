mod point;

use crate::point::{ParsePointError, Point};

fn main() -> Result<(), ParsePointError> {
    let points: Result<Vec<_>, _> = grabinput::from_stdin()
        .map(|x| x.trim().parse::<Point>())
        .collect();

    let points = points?;
    let mut finite_points = point::finite_areas(&points);

    finite_points.sort_by_key(|x| x.1);

    for point in finite_points {
        println!("{:?}", point);
    }

    Ok(())
}
