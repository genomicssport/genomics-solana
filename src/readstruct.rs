#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ReadSNP {
    pub rsid: String,
    pub chromosome: String,
    pub position: String,
    pub genotype: String,
}
