use crate::backtest::order::Order;
use crate::dtypes::Bar;
use crate::Point;

pub trait Strategy<T: Point> {
    fn trade_decision(&mut self, bar: &Bar<T>) -> Option<Order>;
}
