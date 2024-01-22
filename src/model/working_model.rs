use ndarray::Array2;
use serde_json::Value;
use std::pin::Pin;

use crate::dtypes::Utility;
use crate::dtypes::{Point, Sample};
use crate::model::Model;
use crate::pyemb;

struct Params {
    coef: Vec<f64>,
    intercept: f64,
    _pin: std::marker::PhantomPinned,
}

impl Params {
    fn new() -> Pin<Box<Self>> {
        let object = Self {
            coef: vec![],
            intercept: 0.0,
            _pin: std::marker::PhantomPinned,
        };
        Box::pin(object)
    }
}

pub struct WorkingModel {
    model_id: u64,
    params: Pin<Box<Params>>,
}

impl WorkingModel {
    pub fn new() -> Self {
        Self {
            model_id: pyemb::new_model(),
            params: Params::new(),
        }
    }

    fn params_mut(&self) -> &mut Params {
        unsafe { &mut *(std::ptr::addr_of!(*self.params) as *mut Params) }
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
        // let p: Array2<f64> = pyemb::predict(self.model_id, &x);
        // p.column(0).iter().map(|&x| Utility(x)).collect()
        let p2 = x
            .rows()
            .into_iter()
            .map(|v| {
                let u = v
                    .iter()
                    .zip(&self.params.coef)
                    .map(|(x, c)| x * c)
                    .sum::<f64>()
                    + self.params.intercept;
                Utility(u)
            })
            .collect();
        p2
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
        if !std::path::Path::new(&path).exists() {
            panic!("model file not found: {}", path);
        }
        let params = std::fs::read_to_string(path)?;
        pyemb::set_params(self.model_id, &params);
        let params: Value = serde_json::from_str(&params)?;
        self.params_mut().coef = params["coef"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_f64().unwrap())
            .collect();
        self.params_mut().intercept = params["intercept"].as_f64().unwrap();
        Ok(())
    }

    fn loss(&self) -> f64 {
        let params = pyemb::get_params(self.model_id);
        let v: Value = serde_json::from_str(&params).unwrap();
        v.as_object().unwrap()["loss"].as_f64().unwrap_or(f64::NAN)
    }

    fn params(&self) -> String {
        pyemb::get_params(self.model_id)
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
