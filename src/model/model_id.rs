use crate::model::model_type::ModelType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModelId {
    iteration: usize,
    model_type: ModelType,
}
