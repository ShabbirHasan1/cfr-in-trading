mod model;
mod model_id;
mod model_set;
mod model_type;
mod random_model;
mod working_model;

use std::sync::Arc;

pub use model::Model;
pub use model_id::ModelId;
pub use model_set::ModelSet;
pub use model_type::{ModelAction, ModelSide, ModelType};
pub use random_model::RandomModel;
pub use working_model::WorkingModel;

pub type ModelSetRef<T> = Arc<ModelSet<T>>;
