pub type Serie = String; // "1", "2"

#[derive(Debug, Clone)]
pub struct SerieInfo {
    pub serie: u16, // 1, 2
    pub fhd: Option<String>,
    pub hd: Option<String>,
    pub sd: Option<String>,
}
