use crate::dtypes::{Point, Utility};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Sample<T: Point> {
    pub point: T,
    pub utility: Utility,
}
