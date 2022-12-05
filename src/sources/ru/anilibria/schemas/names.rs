use std::fmt::{self, Display};

pub struct Names {
    pub ru: String, // "Девочка-волшебница Мадока★Магика"
    pub en: String, // "Mahou Shoujo Madoka★Magica"
}

impl Display for Names {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{ru} | {en}", ru = self.ru, en = self.en)
    }
}
