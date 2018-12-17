use std::collections::VecDeque;

#[derive(Default)]
pub struct Circle<T> {
    current: usize,
    marbles: VecDeque<T>,
}

impl<T> Circle<T> {
    pub fn push(&mut self, item: T) {
        self.apply_positive_offset(2);
        self.marbles.insert(self.current, item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.apply_negative_offset(7);
        self.marbles.remove(self.current)
    }

    fn apply_negative_offset(&mut self, offset: usize) {
        match self.marbles.len() {
            0 => (),
            len => {
                if self.current > offset {
                    self.current -= offset;
                } else {
                    self.current = len - offset - self.current;
                }
            }
        }
    }

    fn apply_positive_offset(&mut self, offset: usize) {
        match self.marbles.len() {
            0 => (),
            len => {
                if self.current + offset < len {
                    self.current += offset;
                } else {
                    self.current = self.current + offset - len;
                }
            }
        }
    }
}
