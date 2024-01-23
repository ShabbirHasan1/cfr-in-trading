use rand::prelude::ThreadRng;
use rand::Rng;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

use crate::config::IterationConfig;
use crate::dtypes::{DatesetRef, Point, Sample, Utility};
use crate::model::{Model, ModelSet, ModelSetRef, ModelType};
use crate::train::inferrer::Inferrer;
use crate::train::iteration_stat::IterationStat;
use crate::train::play::Play;

/// Runs plays to collect training data and trains models.
pub struct Iteration<T: Point> {
    iteration_index: usize,
    dataset: DatesetRef<T>,
    config: IterationConfig,
    input_model_set: ModelSetRef<T>,
    output_model_set: ModelSetRef<T>,
    stat: Arc<IterationStat>,
}

impl<T: Point> Iteration<T> {
    pub fn new(iteration_index: usize, dataset: DatesetRef<T>, config: IterationConfig) -> Self {
        let input_model_set: ModelSetRef<T> =
            ModelSet::new(iteration_index - 1, &config.output_dir);
        input_model_set.load_model_params();
        let output_model_set = ModelSet::new(iteration_index, &config.output_dir);
        let stat = Arc::new(IterationStat::new());
        Self {
            iteration_index,
            dataset,
            config,
            input_model_set,
            output_model_set,
            stat,
        }
    }

    pub fn run(&self) {
        let samples: Vec<Vec<Sample<T>>> =
            ModelType::all().into_iter().map(|_| Vec::new()).collect();
        let samples = Arc::new(RwLock::new(samples));
        let join_handles = (0..self.config.concurrency)
            .map(|_| {
                let config = self.config.clone();
                let samples2 = samples.clone();
                let dataset = self.dataset.clone();
                let models = self.input_model_set.clone();
                let n_plays = self.config.n_plays as usize; // / self.config.concurrency as usize;
                let stat = self.stat.clone();
                std::thread::spawn(move || {
                    run_plays_sequentially(config, samples2, dataset, models, n_plays, stat)
                })
            })
            .collect::<Vec<_>>();
        join_handles
            .into_iter()
            .for_each(|handle| handle.join().unwrap());
        println!("training");
        self.train_models(samples);
        self.save_models();
    }

    pub fn summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "mean play len: {}\n",
            self.stat.mean_play_length()
        ));
        out.push_str(&format!(
            "mean prediction: {}\n",
            self.stat.mean_prediction(),
        ));
        for model_type in ModelType::all() {
            let model: &Box<dyn Model<T>> = self.output_model_set.model(model_type);
            let params = model.params();
            out.push_str(&format!("{}: {}\n", model_type, params));
        }
        out
    }

    fn train_models(&self, samples: Arc<RwLock<Vec<Vec<Sample<T>>>>>) {
        if samples.read().unwrap().is_empty() {
            return;
        }
        for model_type in ModelType::all() {
            let model: &Box<dyn Model<T>> = self.output_model_set.model(model_type);
            let model_index: usize = model_type.into();
            let modelwise_samples = &samples.read().unwrap()[model_index];
            if modelwise_samples.is_empty() {
                continue;
            }
            model.train(modelwise_samples);
        }
    }

    fn save_models(&self) {
        std::fs::create_dir_all(&self.config.output_dir).expect("failed to create directory");
        for model_type in ModelType::all() {
            let path: String = format!(
                "{}/{}_{}.json",
                &self.config.output_dir, self.iteration_index, model_type
            );
            let model: &Box<dyn Model<T>> = self.output_model_set.model(model_type);
            model.save(&path).unwrap();
        }
    }
}

#[allow(dead_code)]
fn run_plays<T: Point>(
    config: IterationConfig,
    samples: Arc<RwLock<Vec<Vec<Sample<T>>>>>,
    dataset: DatesetRef<T>,
    models: ModelSetRef<T>,
    n_plays: usize,
    stat: Arc<IterationStat>,
) {
    let mut inferrer: Inferrer<T> = Inferrer::new(dataset.clone(), models.clone(), n_plays);
    let mut plays: Vec<Play<T>> = ModelType::all()
        .iter()
        .flat_map(|trained_model_type| {
            (0..n_plays).map(|_| Play::new(&config, dataset.clone(), trained_model_type.clone()))
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
                    // println!("request: {:?}", request);
                    all_finished = false;
                    inferrer.put_request(play_index, request.model_type, request.bar_index);
                }
                None => {
                    // println!("none!");
                }
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
                stat.update_predictions(inference.prediction.0);
                play.advance_with_inference(inference.prediction);
            });
    }
    for play in plays.iter() {
        if !dataset[play.start_bar_index()].point.is_finite() {
            continue;
        }
        let point: T = dataset[play.start_bar_index()].point.clone();
        stat.update_play_lengths(play.len());
        let model_type: ModelType = play.trained_model_type();
        let model_index: usize = model_type.into();
        let utility: Utility = play.utility();
        let sample: Sample<T> = Sample { point, utility };
        samples.write().unwrap()[model_index].push(sample);
    }
}

fn run_plays_sequentially<T: Point>(
    config: IterationConfig,
    samples: Arc<RwLock<Vec<Vec<Sample<T>>>>>,
    dataset: DatesetRef<T>,
    models: ModelSetRef<T>,
    n_plays: usize,
    stat: Arc<IterationStat>,
) {
    let inferrer: Inferrer<T> = Inferrer::new(dataset.clone(), models.clone(), n_plays);
    let mut local_samples: Vec<Vec<Sample<T>>> =
        ModelType::all().into_iter().map(|_| Vec::new()).collect();
    let mut rng: ThreadRng = rand::thread_rng();

    loop {
        if stat.n_plays() >= n_plays {
            break;
        }
        let trained_model_type =
            ModelType::try_from(rng.gen_range(0..ModelType::N_VARIANTS)).unwrap();
        let mut play = Play::new(&config, dataset.clone(), trained_model_type.clone());

        loop {
            if play.is_finished() {
                break;
            }
            match play.advance_to_inference() {
                Some(request) => {
                    let utility = inferrer.infer(request.model_type, request.bar_index);
                    stat.update_predictions(utility.0);
                    play.advance_with_inference(utility);
                }
                None => {
                    break;
                }
            }
        }

        stat.update_play_lengths(play.len());
        let point: T = dataset[play.start_bar_index()].point.clone();
        let model_index: usize = play.trained_model_type().into();
        let utility: Utility = play.utility();
        let sample: Sample<T> = Sample { point, utility };
        local_samples[model_index].push(sample);
    }
    let mut locked_samples: RwLockWriteGuard<Vec<Vec<Sample<T>>>> = samples.write().unwrap();
    ModelType::all().into_iter().for_each(|model_type| {
        let model_index: usize = model_type.into();
        locked_samples[model_index].extend(local_samples[model_index].drain(..));
    });
}
