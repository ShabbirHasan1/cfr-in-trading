use crate::backtest::instrument_id::InstrumentId;

#[derive(Debug, Clone)]
pub struct Order {
    pub instrument_id: InstrumentId,
    pub size: i32,
}
