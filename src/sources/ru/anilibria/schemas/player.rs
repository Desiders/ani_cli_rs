use super::{Serie, SerieInfo, Series};

use std::collections::HashMap;

pub struct Player {
    pub host: String, // "static.libria.fun", "de6.libria.fun"
    pub series: Series,
    pub playlist: HashMap<Serie, SerieInfo>,
}
