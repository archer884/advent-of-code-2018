use crate::point::Point;

pub fn field_iterator(points: &[Point]) -> Option<impl Iterator<Item = Point>> {
    use std::cmp;

    let mut min_x = None;
    let mut max_x = None;
    let mut min_y = None;
    let mut max_y = None;

    for &Point { x, y } in points {
        min_x = Some(min_x.map_or(x, |current| cmp::min(current, x)));
        max_x = Some(max_x.map_or(x, |current| cmp::max(current, x)));
        min_y = Some(min_y.map_or(y, |current| cmp::min(current, y)));
        max_y = Some(max_x.map_or(y, |current| cmp::max(current, y)));
    }

    let min_x = min_x?;
    let max_x = max_x?;
    let min_y = min_y?;
    let max_y = max_y?;

    Some((min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| Point::new(x, y))))
}
