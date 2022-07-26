use super::{
    playlist::{Serie, SerieInfo},
    series::Series,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub host: String, // "static.libria.fun", "de6.libria.fun"
    pub series: Series,
    pub playlist: HashMap<Serie, SerieInfo>,
}
