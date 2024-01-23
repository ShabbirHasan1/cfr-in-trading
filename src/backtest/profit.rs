use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Profit {
    pub timestamp: i64,
    pub profit: f64,
}
