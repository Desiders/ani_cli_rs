use std::fmt::{self, Display};

pub type Serie = String; // "1", "2"

pub struct SerieInfo {
    pub serie: u16, // 1, 2
    pub fhd: Option<String>,
    pub hd: Option<String>,
    pub sd: Option<String>,
}

impl Display for SerieInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{serie}", serie = self.serie)
    }
}
