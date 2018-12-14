use std::fmt::{self, Debug, Display};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Step(pub u8);

impl Step {
    pub fn turns(self) -> i32 {
        i32::from(self.0 - b'A') + 61
    }
}

impl Debug for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_tuple("Step");
        debug.field(&(self.0 as char));
        debug.finish()
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}
