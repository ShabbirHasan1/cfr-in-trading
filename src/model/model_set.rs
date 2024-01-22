use std::sync::Arc;

use crate::dtypes::Point;
use crate::model::{Model, ModelType, RandomModel, WorkingModel};

pub struct ModelSet<T: Point> {
    iteration_index: usize,
    output_dir: String,
    models: Arc<Vec<Box<dyn Model<T>>>>,
}

impl<T: Point> ModelSet<T> {
    pub fn new(iteration_index: usize, output_dir: &str) -> Arc<Self> {
        let models: Vec<Box<dyn Model<T>>> = ModelType::all()
            .into_iter()
            .map(|_model_type| {
                let model: Box<dyn Model<T>> = if iteration_index == 0 {
                    Box::new(RandomModel {})
                } else {
                    Box::new(WorkingModel::new())
                };
                model
            })
            .collect();
        let object = Self {
            iteration_index,
            output_dir: output_dir.to_string(),
            models: Arc::new(models),
        };
        Arc::new(object)
    }

    pub fn model(&self, model_type: ModelType) -> &Box<dyn Model<T>> {
        let model_index: usize = model_type.into();
        &self.models[model_index]
    }

    pub fn len(&self) -> usize {
        self.models.len()
    }

    pub fn load_model_params(&self) {
        ModelType::all().into_iter().for_each(|model_type| {
            let model_path = format!(
                "{}/{}_{}.json",
                self.output_dir, self.iteration_index, model_type
            );
            let model_index: usize = model_type.into();
            self.models[model_index].load(&model_path).unwrap();
        });
    }
}
