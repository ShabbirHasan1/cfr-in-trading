use rand::prelude::ThreadRng;
use rand::Rng;

use crate::dtypes::{Bar, Utility};
use crate::dtypes::{DatesetRef, Point};
use crate::model::{ModelAction, ModelSide, ModelType};

/// Single play (opening and closing trades)
pub struct Play<T: Point> {
    fee: f64,
    dataset: DatesetRef<T>,
    trained_model_type: ModelType,
    closing_model_type: ModelType,
    start_index: usize,
    current_index: usize,
    finished: bool,
}

impl<T: Point> Play<T> {
    pub fn new(fee: f64, dataset: DatesetRef<T>, trained_model_type: ModelType) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let start_index: usize = rng.gen_range(0..dataset.len() - 10);
        let current_index: usize = start_index + 1;
        let closing_model_type: ModelType = ModelType {
            side: trained_model_type.side,
            action: ModelAction::Closing,
        };
        Self {
            fee,
            dataset,
            trained_model_type,
            closing_model_type,
            start_index,
            current_index,
            finished: false,
        }
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    #[inline]
    pub fn start_bar_index(&self) -> usize {
        self.start_index
    }

    #[inline]
    pub fn trained_model_type(&self) -> ModelType {
        self.trained_model_type
    }

    pub fn utility(&self) -> Utility {
        let start_bar: &Bar<T> = &self.dataset[self.start_index];
        let current_bar: &Bar<T> = &self.dataset[self.current_index];
        let price_return: f64 = current_bar.mid_price.0 - start_bar.mid_price.0;
        let sign: f64 = match self.trained_model_type.side {
            ModelSide::Long => 1.0,
            ModelSide::Short => -1.0,
        };
        let fee: f64 = match self.trained_model_type.action {
            ModelAction::Opening => self.fee * 2.0,
            ModelAction::Closing => 0.0,
        };
        let utility: f64 = price_return * sign - fee;
        Utility(utility)
    }

    pub fn advance_to_inference(&mut self) -> Option<InferenceRequest> {
        if self.finished {
            return None;
        }
        if self.current_index >= self.dataset.len() {
            self.finished = true;
            return None;
        }
        Some(InferenceRequest {
            bar_index: self.current_index,
            model_type: self.closing_model_type,
        })
    }

    pub fn advance_with_inference(&mut self, utility: Utility) {
        let utility_of_doing_nothing: f64 = utility.0;
        if utility_of_doing_nothing > 0.0 {
            self.finished = true;
        } else {
            self.current_index += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InferenceRequest {
    pub bar_index: usize,
    pub model_type: ModelType,
}