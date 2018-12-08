use hashbrown::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[cfg(test)]
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn is_finite(self, others: &[Point]) -> bool {
        let mut x_limit_max = false;
        let mut x_limit_min = false;
        let mut y_limit_max = false;
        let mut y_limit_min = false;

        for &Point { x, y } in others.iter().filter(|&&x| x != self) {
            if !x_limit_max {
                x_limit_max = self.x < x;
            }

            if !x_limit_min {
                x_limit_min = self.x > x;
            }

            if !y_limit_max {
                y_limit_max = self.y < y;
            }

            if !y_limit_min {
                y_limit_min = self.y > y;
            }
        }

        x_limit_max && x_limit_min && y_limit_max && y_limit_min
    }

    fn distance(self, Point { x, y }: Point) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

#[derive(Debug)]
pub enum ParsePointError {
    Integer(ParseIntError),
    Other(&'static str),
}

impl From<ParseIntError> for ParsePointError {
    fn from(e: ParseIntError) -> ParsePointError {
        ParsePointError::Integer(e)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(", ").map(|x| x.parse());

        let x = parts.next().ok_or(ParsePointError::Other("Bad format"))??;
        let y = parts.next().ok_or(ParsePointError::Other("Bad format"))??;

        if parts.next().is_some() {
            return Err(ParsePointError::Other("Bad format"));
        }

        Ok(Point { x, y })
    }
}

pub fn finite_areas(points: &[Point]) -> Vec<(Point, i32)> {
    struct FieldSize {
        min_x: i32,
        max_x: i32,
        min_y: i32,
        max_y: i32,
    }

    fn field_bounds(points: &[Point]) -> Option<FieldSize> {
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

        Some(FieldSize {
            min_x: min_x?,
            max_x: max_x?,
            min_y: min_y?,
            max_y: max_y?,
        })
    }

    fn single<T>(items: Vec<T>) -> Option<T> {
        if items.len() == 1 {
            items.into_iter().next()
        } else {
            None
        }
    }

    let FieldSize {
        min_x,
        max_x,
        min_y,
        max_y,
    } = match field_bounds(points) {
        None => return vec![],
        Some(bounds) => bounds,
    };

    let witnesses = (min_x..=max_x).flat_map(|x| (min_y..=max_y).map(move |y| Point { x, y }));

    let mut map = HashMap::new();
    for witness in witnesses {
        let mut points_by_distance = HashMap::new();
        points.iter().cloned().for_each(|point| {
            points_by_distance
                .entry(point.distance(witness))
                .or_insert_with(Vec::new)
                .push(point)
        });

        let nearest_points = match points_by_distance.into_iter().min_by_key(|x| x.0) {
            None => return vec![],
            Some(nearest) => nearest,
        };

        if let Some(nearest) = single(nearest_points.1) {
            *map.entry(nearest).or_insert(0) += 1;
        }
    }

    map.into_iter().filter(|x| x.0.is_finite(points)).collect()
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn distance() {
        let a = Point::new(0, 0);
        let b = Point::new(5, 6);

        assert_eq!(11, a.distance(b));
    }
}
