use crate::dtypes::Point;
use crate::dtypes::Utility;
use crate::model::Model;

pub struct RandomModel {}

impl<T: Point> Model<T> for RandomModel {
    fn infer(&self, points: &[T]) -> Vec<Utility> {
        points
            .iter()
            .map(|_| {
                if rand::random() {
                    Utility(1.0)
                } else {
                    Utility(-1.0)
                }
            })
            .collect()
    }
}
