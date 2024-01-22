use crate::dtypes::Utility;
use crate::model::{Model, ModelType};
use crate::{DatesetRef, ModelSetRef, Point};

#[derive(Debug)]
pub struct Inference {
    pub play_index: usize,
    pub prediction: Utility,
}

pub struct Inferrer<T: Point> {
    dataset: DatesetRef<T>,
    models: ModelSetRef<T>,
    n_plays: usize,
    pub points: Vec<Vec<T>>,           // [n_models][n_samples]
    pub play_indices: Vec<Vec<usize>>, // [[n_models][n_samples]
}

impl<T: Point> Inferrer<T> {
    pub fn new(dataset: DatesetRef<T>, models: ModelSetRef<T>, n_plays: usize) -> Self {
        let points: Vec<Vec<T>> = (0..models.len())
            .map(|_| Vec::with_capacity(n_plays))
            .collect();
        let play_indices: Vec<Vec<usize>> = (0..models.len())
            .map(|_| Vec::with_capacity(n_plays))
            .collect();
        Self {
            dataset,
            models,
            n_plays,
            points,
            play_indices,
        }
    }

    pub fn clear(&mut self) {
        for points in self.points.iter_mut() {
            points.clear();
        }
        for play_indices in self.play_indices.iter_mut() {
            play_indices.clear();
        }
    }

    pub fn infer(&self, model_type: ModelType, bar_index: usize) -> Utility {
        let model: &Box<dyn Model<T>> = &self.models.model(model_type);
        let point: T = self.dataset[bar_index].point.clone();
        model.infer(&[point])[0]
    }

    pub fn put_request(&mut self, play_index: usize, model_type: ModelType, bar_index: usize) {
        let model_index: usize = model_type.into();
        let point: T = self.dataset[bar_index].point.clone();
        self.points[model_index].push(point);
        self.play_indices[model_index].push(play_index);
    }

    pub fn fulfill_all_requests(&mut self) -> Vec<Inference> {
        let mut result: Vec<Inference> = Vec::with_capacity(self.n_plays / 10);
        for model_type in ModelType::all() {
            let model_index: usize = model_type.into();
            let points: &[T] = &self.points[model_index];
            if points.is_empty() {
                continue;
            }
            let model: &Box<dyn Model<T>> = &self.models.model(model_type);
            let predictions: Vec<Utility> = model.infer(points);
            for (play_index, prediction) in self.play_indices[model_index].iter().zip(predictions) {
                result.push(Inference {
                    play_index: *play_index,
                    prediction,
                });
            }
        }
        self.clear();
        result
    }
}
