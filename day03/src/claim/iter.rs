use super::{Claim, Unit};

pub struct Units<'claim> {
    source: &'claim Claim,
    left: u16,
    top: u16,
}

impl<'a> Units<'a> {
    pub fn new(source: &Claim) -> Units {
        Units {
            source,
            left: 0,
            top: 0,
        }
    }
}

impl<'a> Iterator for Units<'a> {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.top >= self.source.height {
            return None;
        }

        if self.left >= self.source.width {
            self.left = 0;
            self.top += 1;
            return self.next();
        }

        let result = Unit {
            from_top: self.top + self.source.from_top,
            from_left: self.left + self.source.from_left,
        };

        self.left += 1;
        Some(result)
    }
}
