use crate::config::IterationConfig;
use crate::dtypes::{DatesetRef, Point, Sample, Utility};
use crate::inferrer::Inferrer;
use crate::model::{Model, ModelSet, ModelSetRef, ModelType};
use crate::play::Play;

/// Runs plays to collect training data and trains models.
pub struct Iteration<T: Point> {
    iteration_index: usize,
    dataset: DatesetRef<T>,
    config: IterationConfig,
    input_model_set: ModelSetRef<T>,
    output_model_set: ModelSetRef<T>,
}

impl<T: Point> Iteration<T> {
    pub fn new(iteration_index: usize, dataset: DatesetRef<T>, config: IterationConfig) -> Self {
        let input_model_set: ModelSetRef<T> = if iteration_index == 0 {
            ModelSet::new(true)
        } else {
            ModelSet::new(false)
        };
        let output_model_set = ModelSet::new(false);
        Self {
            iteration_index,
            dataset,
            config,
            input_model_set,
            output_model_set,
        }
    }

    pub fn run(&self) {
        println!("mining...");
        let samples: Vec<Vec<Sample<T>>> = if self.config.concurrency == 1 {
            run_plays_in_single_thread(
                self.config.fee_per_contract_usd,
                self.dataset.clone(),
                self.input_model_set.clone(),
                self.config.n_plays as usize,
            )
        } else {
            unimplemented!()
        };
        println!("training...");
        self.train_models(samples);
        for model_type in ModelType::all() {
            let model: &Box<dyn Model<T>> = self.output_model_set.model(model_type);
            println!("{}: loss={}", model_type, model.loss());
        }
        self.save_models();
        println!("done");
    }

    pub fn summary(&self) -> String {
        "".to_string()
    }

    fn train_models(&self, samples: Vec<Vec<Sample<T>>>) {
        for model_type in ModelType::all() {
            let model: &Box<dyn Model<T>> = self.output_model_set.model(model_type);
            let model_index: usize = model_type.into();
            let modelwise_samples = &samples[model_index];
            model.train(modelwise_samples);
        }
    }

    fn save_models(&self) {
        if self.config.output_dir.is_none() {
            return;
        }
        for model_type in ModelType::all() {
            let path: String = format!(
                "{}/{}_{}.json",
                self.config.output_dir.as_ref().unwrap(),
                self.iteration_index,
                model_type
            );
            let model: &Box<dyn Model<T>> = self.output_model_set.model(model_type);
            model.save(&path).unwrap();
        }
    }
}

fn run_plays_in_single_thread<T: Point>(
    fee: f64,
    dataset: DatesetRef<T>,
    models: ModelSetRef<T>,
    n_plays: usize,
) -> Vec<Vec<Sample<T>>> {
    let mut inferrer: Inferrer<T> = Inferrer::new(dataset.clone(), models.clone(), n_plays);
    let mut plays: Vec<Play<T>> = ModelType::all()
        .iter()
        .flat_map(|trained_model_type| {
            (0..n_plays).map(|_| Play::new(fee, dataset.clone(), trained_model_type.clone()))
        })
        .collect();
    let mut all_finished: bool;
    loop {
        all_finished = true;
        inferrer.clear();
        for (play_index, play) in plays.iter_mut().enumerate() {
            if play.is_finished() {
                continue;
            }
            match play.advance_to_inference() {
                Some(request) => {
                    all_finished = false;
                    inferrer.put_request(play_index, request.model_type, request.bar_index);
                }
                None => {}
            }
        }
        if all_finished {
            break;
        }
        inferrer
            .fulfill_all_requests()
            .iter()
            .for_each(|inference| {
                let play: &mut Play<T> = &mut plays[inference.play_index];
                play.advance_with_inference(inference.prediction);
            });
    }
    let mut samples: Vec<Vec<Sample<T>>> = ModelType::all()
        .into_iter()
        .map(|_| Vec::with_capacity(n_plays))
        .collect();
    for play in plays {
        let model_type: ModelType = play.trained_model_type();
        let utility: Utility = play.utility();
        let point: T = dataset[play.start_bar_index()].point.clone();
        let sample: Sample<T> = Sample {
            point: point,
            utility: utility,
        };
        let model_index: usize = model_type.into();
        samples[model_index].push(sample);
    }
    samples
}
