use crate::dtypes::{Point, Price};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Bar<T: Point> {
    pub timestamp: i64,
    pub mid_price: Price,
    pub point: T,
}
