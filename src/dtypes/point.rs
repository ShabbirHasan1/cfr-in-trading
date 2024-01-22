/// Array of f64 features at a point of time
pub trait Point
where
    Self: Sized + Clone + Send + Sync + std::fmt::Debug + 'static,
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

    fn is_finite(&self) -> bool {
        self.as_ref().iter().all(|x| x.is_finite())
    }
}
