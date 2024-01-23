use std::cell::RefCell;
use std::rc::Rc;

use crate::backtest::position::Position;
use crate::backtest::strategy::Strategy;
use crate::backtest::Profit;
use crate::{DatasetRef, Point};

pub struct Backtester<T: Point> {
    positions: Rc<RefCell<Vec<Position>>>,
    dataset: DatasetRef<T>,
    strategy: Box<dyn Strategy<T>>,
    realized_profits: Vec<Profit>,
    traded_volume_usd: f64,
}

impl<T: Point> Backtester<T> {
    pub fn new(
        positions: Rc<RefCell<Vec<Position>>>,
        dataset: DatasetRef<T>,
        strategy: Box<dyn Strategy<T>>,
    ) -> Self {
        Self {
            positions,
            dataset,
            strategy,
            realized_profits: Vec::new(),
            traded_volume_usd: 0.0,
        }
    }

    pub fn run(&mut self) {
        for bar in self.dataset.iter() {
            match self.strategy.trade_decision(bar) {
                None => {}
                Some(order) => {
                    self.positions.borrow_mut()[order.instrument_id.index].on_order(order);
                }
            }
            for position in self.positions.borrow_mut().iter_mut() {
                let trade_made: bool = position.on_bar(bar);
                if trade_made {
                    if position.position() == 0 {
                        match position.last_realized_profit() {
                            None => unreachable!(),
                            Some(profit) => {
                                println!("Realized profit: {:?}", profit);
                                self.realized_profits.push(profit)
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn realized_profits(&self) -> &[Profit] {
        &self.realized_profits
    }

    pub fn traded_volume_usd(&self) -> f64 {
        self.positions
            .borrow()
            .iter()
            .map(|p| p.traded_volume_usd())
            .sum()
    }
}
