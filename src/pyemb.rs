#![allow(dead_code)]
use ndarray::Array2;
use std::ffi;
use std::ffi::CString;

#[repr(C)]
struct Array2Ptr {
    data_address: u64,
    dim1: i32,
    dim2: i32,
}

impl Array2Ptr {
    fn new(array: &Array2<f64>) -> Self {
        let data_address = array.as_ptr() as u64;
        let dim1 = array.dim().0 as i32;
        let dim2 = array.dim().1 as i32;
        Self {
            data_address,
            dim1,
            dim2,
        }
    }
}

mod _pyemb {
    use super::Array2Ptr;
    use std::ffi::c_char;

    #[link(name = "pyemb", kind = "dylib")]
    #[allow(dead_code)]
    extern "C" {
        pub(super) fn new_model() -> u64;
        pub(super) fn delete_model(model_id: u64);
        pub(super) fn fit(model_id: u64, x: Array2Ptr, y: Array2Ptr);
        pub(super) fn predict(output: Array2Ptr, model_id: u64, x: Array2Ptr);
        pub(super) fn get_params(model_id: u64, output: *const c_char);
        pub(super) fn set_params(model_id: u64, params: *const c_char);
    }
}

pub fn new_model() -> u64 {
    unsafe { _pyemb::new_model() }
}

pub fn delete_model(model_id: u64) {
    unsafe { _pyemb::delete_model(model_id) }
}

pub fn fit(model_id: u64, x: &Array2<f64>, y: &Array2<f64>) {
    let x_ptr = Array2Ptr::new(x);
    let y_ptr = Array2Ptr::new(y);
    unsafe { _pyemb::fit(model_id, x_ptr, y_ptr) }
}

pub fn predict(model_id: u64, x: &Array2<f64>) -> Array2<f64> {
    let x_ptr = Array2Ptr::new(x);
    let n_samples = x.shape()[0];
    let mut output = Array2::zeros((n_samples, 1));
    output.fill(f64::NAN);
    let output_ptr = Array2Ptr::new(&output);
    unsafe { _pyemb::predict(output_ptr, model_id, x_ptr) }
    output
}

pub fn get_params(model_id: u64) -> String {
    let mut output: Vec<u8> = Vec::with_capacity(1024);
    for _ in 0..1024 {
        output.push(0);
    }
    let output = unsafe { CString::from_vec_unchecked(output) };
    let c_str = output.as_ptr();
    unsafe { _pyemb::get_params(model_id, c_str) };
    let output = unsafe { ffi::CStr::from_ptr(output.as_ptr() as *const _) }
        .to_str()
        .unwrap()
        .to_string();
    output
}

pub fn set_params(model_id: u64, params: &str) {
    let params = std::ffi::CString::new(params).unwrap();
    unsafe { _pyemb::set_params(model_id, params.as_ptr()) }
}
