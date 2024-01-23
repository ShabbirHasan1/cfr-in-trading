use std::collections::VecDeque;

use crate::backtest::instrument_id::InstrumentId;
use crate::backtest::instrument_spec::InstrumentSpec;
use crate::backtest::order::Order;
use crate::backtest::Profit;
use crate::dtypes::Bar;
use crate::Point;

pub struct Position {
    instrument_id: InstrumentId,
    instrument_spec: InstrumentSpec,
    orders: VecDeque<Order>,
    avg_price: f64,
    position: i32,
    realized_profits: Vec<Profit>,
    realized_profit: f64,
    traded_volume_usd: f64,
}

impl Position {
    pub fn new(instrument_id: InstrumentId, instrument_spec: InstrumentSpec) -> Self {
        Self {
            instrument_id,
            instrument_spec,
            orders: VecDeque::new(),
            avg_price: 0.0,
            position: 0,
            realized_profits: Vec::new(),
            realized_profit: 0.0,
            traded_volume_usd: 0.0,
        }
    }

    pub fn on_order(&mut self, order: Order) {
        if order.instrument_id != self.instrument_id {
            panic!("Order instrument_id does not match position instrument_id");
        }
        self.orders.push_back(order);
    }

    /// Returns true if a trade was made.
    pub fn on_bar<T: Point>(&mut self, bar: &Bar<T>) -> bool {
        match self.orders.pop_front() {
            None => false,
            Some(order) => {
                let order_size: i32 = order.size;
                self.traded_volume_usd +=
                    order_size.abs() as f64 * bar.mid_price.0 * self.instrument_spec.multiplier;
                let is_closing: bool = self.position.signum() == -order_size.signum();
                if is_closing {
                    let abs_position_before_update: i32 = self.position.abs();
                    let abs_execution_size: i32 = order_size.abs();
                    let abs_closed_size: i32 = abs_position_before_update.min(abs_execution_size);
                    let position_sign: i32 = self.position.signum();
                    let entry_price: f64 = self.avg_price;
                    let exit_price: f64 = bar.mid_price.0; // TODO: can be something more sofisticated
                    let multiplier: f64 = self.instrument_spec.multiplier;
                    let profit: f64 = multiplier
                        * (abs_closed_size as f64)
                        * position_sign as f64
                        * (exit_price - entry_price)
                        - self.instrument_spec.fee * (abs_closed_size as f64);
                    self.realized_profit += profit;
                    let profit_pt = Profit {
                        timestamp: bar.timestamp,
                        profit,
                    };
                    self.realized_profits.push(profit_pt);
                    let closed_size: i32 = abs_closed_size * (-position_sign);
                    let open_size: i32 = self.position + order_size;
                    self.position += closed_size;
                    if self.position == 0 {
                        if open_size == 0 {
                            self.avg_price = 0.0;
                        } else {
                            self.position += open_size;
                            self.avg_price = exit_price;
                        }
                    }
                } else {
                    let abs_position_before_update: f64 = self.position.abs() as f64;
                    let execution_price: f64 = bar.mid_price.0; // TODO: can be something more sofisticated
                    let abs_execution_size: f64 = order_size.abs() as f64;
                    let avp: f64 = self.avg_price;
                    self.position += order_size;
                    if self.position == 0 {
                        self.avg_price = 0.0;
                    } else {
                        self.avg_price = (avp * abs_position_before_update
                            + execution_price * abs_execution_size)
                            / (abs_position_before_update + abs_execution_size);
                    }
                }
                true
            }
        }
    }

    pub fn position(&self) -> i32 {
        self.position
    }

    pub fn realized_profits(&self) -> &[Profit] {
        &self.realized_profits
    }

    pub fn realized_profit(&self) -> f64 {
        self.realized_profit
    }

    pub fn last_realized_profit(&self) -> Option<Profit> {
        self.realized_profits.last().cloned()
    }

    pub fn instrument_id(&self) -> &InstrumentId {
        &self.instrument_id
    }

    pub fn traded_volume_usd(&self) -> f64 {
        self.traded_volume_usd
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dtypes::Price;

    #[derive(Debug, Clone)]
    struct P {}
    impl Point for P {}

    #[test]
    fn test() {
        let bars = [
            Bar {
                mid_price: Price(100.0),
                point: P {},
            },
            Bar {
                mid_price: Price(101.0),
                point: P {},
            },
            Bar {
                mid_price: Price(102.0),
                point: P {},
            },
        ];
        let instrument_id = InstrumentId {
            index: 0,
            symbol: "".to_string(),
        };
        let instrument_spec = InstrumentSpec {
            multiplier: 20.0,
            fee: 1.5,
        };
        let mut position = Position::new(instrument_id.clone(), instrument_spec);
        let order1 = Order {
            instrument_id: instrument_id.clone(),
            size: 1,
        };
        let order2 = Order {
            instrument_id: instrument_id.clone(),
            size: -2,
        };
        position.on_order(order1.clone());
        position.on_bar(&bars[0]);
        position.on_order(order1.clone());
        position.on_bar(&bars[1]);
        position.on_order(order2.clone());
        position.on_bar(&bars[2]);
        let expected_profit = 57.0;
        let profit = position.realized_profit();
        println!("profit: {}", profit);
        assert!((expected_profit - profit).abs() < 1e-6);
    }
}
