mod backtest;
mod config;
pub mod datasets;
mod dtypes;
mod model;
mod pyemb;
pub mod strategies;
mod train;
mod utils;

pub use config::Config;
pub use dtypes::{DatasetRef, Point};
pub use model::{ModelSet, ModelSetRef, ModelType};
pub use train::Iteration;

pub use backtest::{Backtester, InstrumentId, InstrumentSpec, Position, Profit, Strategy};
