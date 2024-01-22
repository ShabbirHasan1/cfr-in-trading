use atomic_float::AtomicF64;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct IterationStat {
    play_length_sum: AtomicU64,
    n_plays: AtomicU64,
    prediction_sum: AtomicF64,
    n_predictions: AtomicU64,
}

impl IterationStat {
    pub fn new() -> Self {
        Self {
            play_length_sum: AtomicU64::new(0),
            n_plays: AtomicU64::new(0),
            prediction_sum: AtomicF64::new(0.0),
            n_predictions: AtomicU64::new(0),
        }
    }

    pub fn n_plays(&self) -> usize {
        self.n_plays.load(Ordering::Relaxed) as usize
    }

    pub fn update_play_lengths(&self, play_length: usize) {
        self.play_length_sum
            .fetch_add(play_length as u64, Ordering::Acquire);
        self.n_plays.fetch_add(1, Ordering::Acquire);
    }

    pub fn mean_play_length(&self) -> f64 {
        let play_length_sum = self.play_length_sum.load(Ordering::Relaxed);
        let n_plays = self.n_plays.load(Ordering::Relaxed);
        if n_plays == 0 {
            0.0
        } else {
            play_length_sum as f64 / n_plays as f64
        }
    }

    pub fn update_predictions(&self, prediction: f64) {
        self.prediction_sum.fetch_add(prediction, Ordering::Acquire);
        self.n_predictions.fetch_add(1, Ordering::Acquire);
    }

    pub fn mean_prediction(&self) -> f64 {
        let prediction_sum = self.prediction_sum.load(Ordering::Relaxed);
        let n_predictions = self.n_predictions.load(Ordering::Relaxed);
        if n_predictions == 0 {
            0.0
        } else {
            prediction_sum / n_predictions as f64
        }
    }
}
