use ndarray::Array2;
use serde_json::Value;

use crate::dtypes::Utility;
use crate::dtypes::{Point, Sample};
use crate::model::Model;
use crate::pyemb;

pub struct WorkingModel {
    model_id: u64,
}

impl WorkingModel {
    pub fn new() -> Self {
        Self {
            model_id: pyemb::new_model(),
        }
    }
}

impl Drop for WorkingModel {
    fn drop(&mut self) {
        pyemb::delete_model(self.model_id);
    }
}

impl<T: Point> Model<T> for WorkingModel {
    fn infer(&self, points: &[T]) -> Vec<Utility> {
        let x: Array2<f64> = points_to_arr2(points);
        let p: Array2<f64> = pyemb::predict(self.model_id, &x);
        p.column(0).iter().map(|&x| Utility(x)).collect()
    }

    fn train(&self, samples: &[Sample<T>]) {
        let points: Vec<T> = samples.iter().map(|s| s.point.clone()).collect();
        let x: Array2<f64> = points_to_arr2(&points);
        let y: Vec<f64> = samples.iter().map(|s| s.utility.0).collect();
        let y = Array2::from_shape_vec((y.len(), 1), y).unwrap();
        pyemb::fit(self.model_id, &x, &y);
    }

    fn save(&self, path: &str) -> anyhow::Result<()> {
        let params = pyemb::get_params(self.model_id);
        std::fs::write(path, params).map_err(|e| e.into())
    }

    fn load(&self, path: &str) -> anyhow::Result<()> {
        let params = std::fs::read_to_string(path)?;
        pyemb::set_params(self.model_id, &params);
        Ok(())
    }

    fn loss(&self) -> f64 {
        let v: Value = serde_json::from_str(&pyemb::get_params(self.model_id)).unwrap();
        v.as_object().unwrap()["loss"].as_f64().unwrap()
    }
}

fn points_to_arr2<T: Point>(points: &[T]) -> Array2<f64> {
    let point_size_in_f64: usize = std::mem::size_of::<T>() / std::mem::size_of::<f64>();
    let arr_size_in_f64: usize = points.len() * point_size_in_f64;
    let arr_f64: Vec<f64> = unsafe {
        std::ptr::slice_from_raw_parts(points.as_ptr() as *const u8 as *const f64, arr_size_in_f64)
            .as_ref()
    }
    .unwrap()
    .to_vec();
    Array2::from_shape_vec((points.len(), point_size_in_f64), arr_f64).unwrap()
}
