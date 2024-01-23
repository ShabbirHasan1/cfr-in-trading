use crate::dtypes::{Bar, Price};
use crate::{dtypes, utils};
use std::io::Read;
use std::sync::Arc;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Point {
    pub f1: f64,
    pub f2: f64,
    pub f4: f64,
}

impl dtypes::Point for Point {}
unsafe impl Sync for Point {}
unsafe impl Send for Point {}

#[repr(C)]
struct Row {
    mp: f64,
    f1: f64,
    f2: f64,
    f4: f64,
    ts: i64,
}

pub fn load(path: &str, offset: f64, limit: f64) -> dtypes::DatasetRef<Point> {
    let path = utils::canonicalize_path(path).expect("failed to canonicalize path");
    let file = std::fs::File::open(path).expect("failed to open file");
    let file_size = file.metadata().expect("failed to get metadata").len();
    let row_size = std::mem::size_of::<Row>() as u64;
    let n_rows = file_size / row_size;
    let mut dataset: Vec<Bar<Point>> = Vec::with_capacity(n_rows as usize);
    let mut row: Row = unsafe { std::mem::zeroed() };
    let row_slice: &mut [u8] = unsafe {
        std::slice::from_raw_parts_mut(&mut row as *mut Row as *mut u8, std::mem::size_of::<Row>())
    };
    let mut reader = std::io::BufReader::new(file);
    while let Ok(_) = reader.read_exact(row_slice) {
        let bar = Bar {
            timestamp: row.ts,
            mid_price: Price(row.mp),
            point: Point {
                f1: row.f1,
                f2: row.f2,
                f4: row.f4,
            },
        };
        dataset.push(bar);
    }
    let offset = (offset * n_rows as f64) as usize;
    let limit = (limit * n_rows as f64) as usize;
    let dataset = dataset[offset..offset + limit].to_vec();
    Arc::new(dataset)
}
