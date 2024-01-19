/// Array of f64 features at a point of time
pub trait Point
where
    Self: Sized + Clone + std::fmt::Debug,
{
    fn as_ref(&self) -> &[f64] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const f64,
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
