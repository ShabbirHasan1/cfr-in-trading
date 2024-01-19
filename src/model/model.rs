use crate::dtypes::{Point, Sample, Utility};

pub trait Model<T: Point> {
    /// Returns the utility of non-trivial action.
    /// At zero position, non-trivial action is opening
    /// At non-zero position, non-trivial action is closing
    fn infer(&self, points: &[T]) -> Vec<Utility>;

    fn train(&self, _samples: &[Sample<T>]) {
        unimplemented!()
    }

    fn save(&self, _path: &str) -> anyhow::Result<()> {
        Ok(())
    }

    fn load(&self, _path: &str) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn loss(&self) -> f64 {
        unimplemented!()
    }
}