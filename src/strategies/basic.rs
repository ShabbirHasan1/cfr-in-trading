use crate::backtest::{Order, Position, Strategy};
use crate::dtypes::Bar;
use crate::model::{Model, ModelAction, ModelSide, ModelType};
use crate::{ModelSetRef, Point};
use std::cell::RefCell;
use std::rc::Rc;

pub struct BasicStrategy<T: Point> {
    positions: Rc<RefCell<Vec<Position>>>,
    model_set: ModelSetRef<T>,
}

impl<T: Point> BasicStrategy<T> {
    pub fn new(positions: Rc<RefCell<Vec<Position>>>, model_set: ModelSetRef<T>) -> Self {
        Self {
            positions,
            model_set,
        }
    }
}

impl<T: Point> Strategy<T> for BasicStrategy<T> {
    fn trade_decision(&mut self, bar: &Bar<T>) -> Option<Order> {
        let position: i32 = self.positions.borrow()[0].position();
        let instrument_id = self.positions.borrow()[0].instrument_id().clone();
        if position == 0 {
            let open_long_model_type = ModelType {
                side: ModelSide::Long,
                action: ModelAction::Opening,
            };
            let open_long_model: &Box<dyn Model<T>> = self.model_set.model(open_long_model_type);
            let open_long_utility: f64 = open_long_model.infer(&[bar.point.clone()])[0].0;
            let open_short_model_type = ModelType {
                side: ModelSide::Short,
                action: ModelAction::Opening,
            };
            let open_short_model: &Box<dyn Model<T>> = self.model_set.model(open_short_model_type);
            let open_short_utility: f64 = open_short_model.infer(&[bar.point.clone()])[0].0;
            if open_long_utility > 0.0 && open_long_utility >= open_short_utility {
                Some(Order {
                    instrument_id: instrument_id.clone(),
                    size: 1,
                })
            } else if open_short_utility > 0.0 {
                Some(Order {
                    instrument_id: instrument_id.clone(),
                    size: -1,
                })
            } else {
                None
            }
        } else {
            let close_model_type = ModelType {
                side: if position > 0 {
                    ModelSide::Long
                } else {
                    ModelSide::Short
                },
                action: ModelAction::Closing,
            };
            let close_model: &Box<dyn Model<T>> = self.model_set.model(close_model_type);
            let utility_of_doing_nothing: f64 = close_model.infer(&[bar.point.clone()])[0].0;
            if utility_of_doing_nothing > 0.0 {
                None
            } else {
                Some(Order {
                    instrument_id: instrument_id.clone(),
                    size: -position,
                })
            }
        }
    }
}
