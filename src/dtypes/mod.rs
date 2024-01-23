mod bar;
mod point;
mod price;
mod sample;
mod utility;

pub use bar::Bar;
pub use point::Point;
pub use price::Price;
pub use sample::Sample;
pub use utility::Utility;

use std::sync::Arc;

pub type DatasetRef<T> = Arc<Vec<Bar<T>>>;
