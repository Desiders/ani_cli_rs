use std::fmt::{self, Display};

pub struct Series {
    pub first: u16,     // 1, 1
    pub last: u16,      // 24, 1
    pub string: String, // "1-24", "Фильм"
}

impl Display for Series {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{string}", string = self.string)
    }
}
