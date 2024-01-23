#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstrumentId {
    pub symbol: String,
    pub index: usize,
}
