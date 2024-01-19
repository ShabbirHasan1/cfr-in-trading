use std::sync::Arc;

use crate::dtypes::Point;
use crate::model::{Model, ModelType, RandomModel, WorkingModel};

pub struct ModelSet<T: Point> {
    models: Vec<Box<dyn Model<T>>>,
}

impl<T: Point> ModelSet<T> {
    pub fn new(random: bool) -> Arc<Self> {
        let mut object = Self {
            models: Vec::with_capacity(ModelType::N_VARIANTS),
        };
        ModelType::all().into_iter().for_each(|_| {
            let model: Box<dyn Model<T>> = if random {
                Box::new(RandomModel {})
            } else {
                Box::new(WorkingModel::new())
            };
            object.models.push(model);
        });
        Arc::new(object)
    }

    pub fn model(&self, model_type: ModelType) -> &Box<dyn Model<T>> {
        let model_index: usize = model_type.into();
        &self.models[model_index]
    }

    pub fn len(&self) -> usize {
        self.models.len()
    }
}
