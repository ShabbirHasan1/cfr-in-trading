use crate::dtypes::{Point, Price};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Bar<T: Point> {
    pub mid_price: Price,
    pub point: T,
}
